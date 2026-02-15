dev-up:
    docker-compose -f docker-compose.dev.yaml up

dev-down:
    docker-compose -f docker-compose.dev.yaml down

dev-clean:
    docker-compose -f docker-compose.dev.yaml down --rmi all --volumes --remove-orphans

test-up:
    docker-compose -f docker-compose.test.yaml up
    docker-compose -f docker-compose.test.yaml down

test-clean:
    docker-compose -f docker-compose.test.yaml down --rmi all --volumes --remove-orphans
