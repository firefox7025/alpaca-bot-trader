podTemplate(yaml: """
apiVersion: v1
kind: Pod
metadata:
  labels:
    some-label: builder
spec:
  containers:
  - name: rust
    image: rust:1.39.0-slim
    command:
    - cat
    tty: true
"""
  ) {

  node(POD_LABEL) {
    stage('Build and test') {
    checkout scm
      container('rust') {
        sh 'cargo build --release && cargo test'
      }
    }
  }
}pipeline {
    agent {
        docker { image 'rust:1-slim' }
    }
    stages {
        stage('Build') {
            steps {
                sh "cargo build --release"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
    }
}