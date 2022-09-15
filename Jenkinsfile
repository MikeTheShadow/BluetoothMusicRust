pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'cargo build --release'
        archiveArtifacts artifacts: '**/target/release/BluetoothMusicRust', fingerprint: true
      }
    }
  }
}