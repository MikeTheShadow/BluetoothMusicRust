pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh '/home/jenkins/.cargo/bin/cargo build --target=armv7-unknown-linux-gnueabihf --release'
        archiveArtifacts artifacts: '**/target/release/BluetoothMusicRust', fingerprint: true
      }
    }
  }
}