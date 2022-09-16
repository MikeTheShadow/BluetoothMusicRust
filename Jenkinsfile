pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh '/home/jenkins/.cargo/bin/cargo build --release'
        archiveArtifacts artifacts: '**/target/release/BluetoothMusicRust', fingerprint: true
      }
    }
  }
}