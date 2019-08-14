pipeline {
    agent {
        docker { image 'rust:1-stretch' }
    }
    stages {
        stage('Build') {
            steps {
                sh cargo build --release
            }
        }
        stage('Test') {
            steps {
                cargo test
            }
        }
        stage('Deploy') {
            steps {
                echo 'Deploying....'
            }
        }
    }
}