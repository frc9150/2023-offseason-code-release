use std::ops::Neg;

use nalgebra::{
    allocator::Allocator, ClosedAdd, ClosedMul, Complex, DefaultAllocator, DimMul, DimProd, Matrix, OMatrix, Owned, RawStorage, RealField, ReshapableStorage, RowVector3, Scalar,
    SimdRealField, UnitComplex, Vector2, Vector3, U1, U2, U3, Vector, Storage,
};
use num_traits::{One, Zero};

#[derive(Debug)]
pub struct ChassisSpeeds<T> {
    pub linear_vel: Vector2<T>,
    pub angular_vel: T,
}

impl<T> ChassisSpeeds<T> {
    pub fn from_field_rel(
        linear_vel: Vector2<T>,
        angular_vel: T,
        field_to_robot: UnitComplex<T>,
    ) -> Self
    where
        T: SimdRealField,
        T::Element: SimdRealField,
    {
        Self {
            linear_vel: field_to_robot * linear_vel,
            angular_vel,
        }
    }

    pub fn discretize<S>(linear_vel: Vector<T, U2, S>, angular_vel: T, time: T) -> Self
    where
        T: RealField + Copy,
        S: Storage<T, U2>,
    {
        // https://github.com/wpilibsuite/allwpilib/blob/a74db52dae0edfcd481db3324be5ce9014120dae/wpimath/src/main/java/edu/wpi/first/math/kinematics/ChassisSpeeds.java#L92
        // https://github.com/wpilibsuite/allwpilib/blob/main/wpimath/src/main/java/edu/wpi/first/math/geometry/Pose2d.java#L249
        // TODO: understand the code
        // Related to SE(2) (see chapter 10 of https://file.tavsys.net/control/controls-engineering-in-frc.pdf)
        let (dpos, dtheta) = (linear_vel * time, angular_vel * time);
        let half_dtheta = dtheta / (T::one() + T::one());
        let cos_minus_one = dtheta.cos() - T::one();

        let e9 = T::from_f64(1e-9);
        let twelve = if e9.is_some() { T::from_u8(12) } else { None };

        let half_theta_by_tan_of_half_dtheta = if let (Some(e9), Some(twelve)) = (e9, twelve) {
            if cos_minus_one.abs() < e9 {
                T::one() - T::one() / twelve * dtheta * dtheta
            } else {
                -(half_dtheta * dtheta.sin()) / cos_minus_one
            }
        } else {
            -(half_dtheta * dtheta.sin()) / cos_minus_one
        };

        // This can likely be simplified
        let translation = (UnitComplex::from_complex(Complex::new(
            half_theta_by_tan_of_half_dtheta,
            -half_dtheta,
        )) * dpos)
            * T::hypot(half_theta_by_tan_of_half_dtheta, half_dtheta);
        Self {
            linear_vel: translation / time,
            angular_vel,
        }
    }
}

pub struct SwerveKinematics<T, N>
where
    N: DimMul<U2>,
    DefaultAllocator: Allocator<T, DimProd<N, U2>, U3>,
{
    inverse: OMatrix<T, DimProd<N, U2>, U3>,
    size: N,
}

impl<T, N> SwerveKinematics<T, N>
where
    T: Scalar + Zero + One,
    N: DimMul<U2>,
    DefaultAllocator: Allocator<T, DimProd<N, U2>, U3>,
{
    pub fn new<S>(module_translations: &Matrix<T, U2, N, S>) -> Self
    where
        T: Neg<Output = T>,
        S: RawStorage<T, U2, N>,
    {
        let shape = module_translations.shape_generic();
        let mut inverse = OMatrix::zeros_generic(N::mul(shape.1, U2), U3);
        for (i, module) in module_translations.column_iter().enumerate() {
            inverse.set_row(
                i * 2,
                &RowVector3::new(T::one(), T::zero(), -module[1].clone()),
            );
            inverse.set_row(
                i * 2 + 1,
                &RowVector3::new(T::zero(), T::one(), module[0].clone()),
            );
        }
        Self {
            inverse,
            size: shape.1,
        }
    }

    /// Takes robot-relative linear and angular velocities, and returns robot-relative wheel
    /// velocities as a 2xN matrix.
    pub fn to_module_states(
        &self,
        linear_vel: Vector2<T>,
        angular_vel: T,
    ) -> Matrix<
        T,
        U2,
        N,
        <Owned<T, DimProd<N, U2>> as ReshapableStorage<T, DimProd<N, U2>, U1, U2, N>>::Output,
    >
    where
        T: ClosedAdd + ClosedMul,
        Owned<T, DimProd<N, U2>>: ReshapableStorage<T, DimProd<N, U2>, U1, U2, N>,
        DefaultAllocator: Allocator<T, DimProd<N, U2>>,
    {
        let states_flat =
            &self.inverse * Vector3::new(linear_vel.x.clone(), linear_vel.x.clone(), angular_vel);
        states_flat.reshape_generic::<U2, N>(U2, self.size)
    }
}
