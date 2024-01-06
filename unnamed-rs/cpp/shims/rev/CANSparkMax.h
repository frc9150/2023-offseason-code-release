#include <rev/CANSparkMaxLowLevel.h>
#include <rev/CANSparkMax.h>

namespace rev {
	inline std::unique_ptr<CANSparkMax> CANSparkMax_ctor(int deviceID, CANSparkMax::MotorType type) {
		return std::make_unique<CANSparkMax>(deviceID, type);
	}

	inline std::unique_ptr<SparkMaxPIDController> CANSparkMax_GetPIDController(CANSparkMax &motor) {
		return std::unique_ptr<SparkMaxPIDController>(new SparkMaxPIDController(motor.GetPIDController()));
	}
}
