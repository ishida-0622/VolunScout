db-up:
	docker compose -f ./docker/docker-compose-database.yml up -d
	timeout 15
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -uvolunscout -ptuocsnulov volunscout < .\sql\up.sql

db-down:
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -uvolunscout -ptuocsnulov volunscout < .\sql\down.sql
	timeout 3
	docker compose -f ./docker/docker-compose-database.yml down

db-reset:
	db-down
	timeout 10
	db-up

db-init-insert:
	docker compose -f ./docker/docker-compose-database.yml exec -T db mysql -uvolunscout -ptuocsnulov volunscout < .\sql\init-insert.sql
