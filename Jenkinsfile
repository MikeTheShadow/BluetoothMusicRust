pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh '/home/jenkins/.cargo/bin/cargo build --target=arm-none-linux-gnueabihf-gcc --release'
        archiveArtifacts artifacts: '**/target/release/BluetoothMusicRust', fingerprint: true
      }
    }
  }
}