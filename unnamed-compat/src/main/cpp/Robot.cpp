// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

#include "Robot.h"
/*//extern "C" {
#include <nng/nng.h>
#include <nng/protocol/reqrep0/rep.h>
#include <nng/protocol/reqrep0/req.h>
#include <nng/supplemental/util/platform.h>
#include <nng/transport/tcp/tcp.h>
//}*/
#include <iostream>
#include <dlfcn.h>
#include <frc/Filesystem.h>

#include <frc/DriverStation.h>
#include <frc/internal/DriverStationModeThread.h>
#include <frc/livewindow/LiveWindow.h>
#include <frc/shuffleboard/Shuffleboard.h>
#include <hal/DriverStation.h>
#include <networktables/NetworkTable.h>

#include <rev/CANSparkMax.h>

void Robot::RobotInit() {}

void Robot::Disabled() {}

void Robot::Autonomous() {}

void Robot::Teleop() {}

void Robot::Test() {}

void Robot::StartCompetition() {
  RobotInit();

  frc::internal::DriverStationModeThread modeThread;

  wpi::Event event{false, false};
  frc::DriverStation::ProvideRefreshedDataEventHandle(event.GetHandle());

  // Tell the DS that the robot is ready to be enabled
  HAL_ObserveUserProgramStarting();

  fprintf(stderr, "about to do thingy\n");
  if (dl_entry == NULL) {
	  fprintf(stderr, "uh oh\n");
	  fprintf(stderr, "%s \n", dlerror());
  }
  dl_entry();
  // error checking is for dweebs
  // TODO: error checking
  /*nng_tcp_register();

  nng_socket sock;
  nng_rep0_open(&sock);

  nng_listener listener;
  int rv = nng_listener_create(&listener, sock, "tcp://localhost:9150");
  fprintf(stderr, "%s: %s\n", "hi", nng_strerror(rv));
  nng_socket_set_ms(sock, NNG_OPT_REQ_RESENDTIME, 2000);
  nng_listener_start(listener, 0);

  while (true) {
    char * buf = NULL;
    size_t sz;
    nng_recv(sock, &buf, &sz, NNG_FLAG_ALLOC);
    if (sz == 1 && buf[0] == 1) {
        frc::DriverStation::Alliance all = frc::DriverStation::GetAlliance();
        char out = 0;
        if (all == frc::DriverStation::Alliance::kBlue) {
            out = 1;
        }
        buf[0] = out;
        nng_send(sock, buf, sz, NNG_FLAG_ALLOC);
    }
  }*/

  /*while (!m_exit) {
    if (IsDisabled()) {
      modeThread.InDisabled(true);
      Disabled();
      modeThread.InDisabled(false);
      nng_socket sock;
      nng_rep0_open(&sock);
      while (IsDisabled()) {
        wpi::WaitForObject(event.GetHandle());
        std::cout << "hi\n";
      }
    } else if (IsAutonomous()) {
      modeThread.InAutonomous(true);
      Autonomous();
      modeThread.InAutonomous(false);
      while (IsAutonomousEnabled()) {
        wpi::WaitForObject(event.GetHandle());
      }
    } else if (IsTest()) {
      frc::LiveWindow::SetEnabled(true);
      frc::Shuffleboard::EnableActuatorWidgets();
      modeThread.InTest(true);
      Test();
      modeThread.InTest(false);
      while (IsTest() && IsEnabled()) {
        wpi::WaitForObject(event.GetHandle());
      }
      frc::LiveWindow::SetEnabled(false);
      frc::Shuffleboard::DisableActuatorWidgets();
    } else {
      modeThread.InTeleop(true);
      Teleop();
      modeThread.InTeleop(false);
      while (IsTeleopEnabled()) {
        wpi::WaitForObject(event.GetHandle());
      }
    }
  }*/
}

void Robot::EndCompetition() {
  m_exit = true;
}

extern "C" {
  uint8_t get_alliance() {
    frc::DriverStation::Alliance alliance = frc::DriverStation::GetAlliance();
    switch (alliance) {
      case frc::DriverStation::Alliance::kRed:
        return 0;
      case frc::DriverStation::Alliance::kBlue:
        return 1;
      default: // kInvalid
        return 2;
    }
  }
  void dummy() {
	  rev::CANSparkMax a{0, rev::CANSparkMax::MotorType::kBrushless};
  }
}

#ifndef RUNNING_FRC_TESTS
int main() {
  //dlopen(NULL, RTLD_GLOBAL);
  dl = dlopen((frc::filesystem::GetDeployDirectory() + "/libfrc.so").c_str(), RTLD_LAZY);
  if (dl == NULL) fprintf(stderr, "oops\n");
  fprintf(stderr, "%s \n", dlerror());
  dl_entry = (void (*)())dlsym(dl, "main_rs");
  return frc::StartRobot<Robot>();
}
#endif
