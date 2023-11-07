include ../.env

db-up:
	docker compose -f ./docker/docker-compose-database.yml up -d
	timeout 15
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\up.sql

db-down:
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\down.sql
	timeout 1
	docker compose -f ./docker/docker-compose-database.yml down

db-reset:
	db-down
	timeout 10
	db-up

db-init-insert:
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\init-insert.sql
