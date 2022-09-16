pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh '/home/jenkins/.cargo/bin/cargo build --target=aarch64-unknown-linux-gnu --release'
        archiveArtifacts artifacts: '**/target/release/BluetoothMusicRust', fingerprint: true
      }
    }
  }
}