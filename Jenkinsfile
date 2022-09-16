pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        source "$HOME/.cargo/env"
        sh 'cargo build --release'
        archiveArtifacts artifacts: '**/target/release/BluetoothMusicRust', fingerprint: true
      }
    }
  }
}