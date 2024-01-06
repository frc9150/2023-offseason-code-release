#include <frc/DriverStation.h>

namespace frc {
	typedef frc::DriverStation::Alliance DriverStation_Alliance;

	inline DriverStation_Alliance DriverStation_GetAlliance() {
		return frc::DriverStation::GetAlliance();
	}
}
