# Windows環境でのみ動作確認済み. Linux, Macの場合はtimeoutコマンドをsleepコマンドに変更する必要がある

include ../.env

db-up:
	docker compose -f ./docker/docker-compose-database.yml up -d
	timeout 20
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\up.sql

db-down:
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\down.sql
	docker compose -f ./docker/docker-compose-database.yml down

db-init-insert:
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\init-insert.sql

db-up-insert:
	docker compose -f ./docker/docker-compose-database.yml up -d
	timeout 20
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\up.sql
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -u$(MYSQL_USER_NAME) -p$(MYSQL_PASSWORD) $(MYSQL_DATABASE) < .\sql\init-insert.sql
