pipeline {
    agent {
        docker { image 'rust:1-stretch' }
    }
    stages {
        stage('Build') {
            step {
                cargo build --release
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