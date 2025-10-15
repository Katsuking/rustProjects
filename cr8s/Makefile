.PHONY: up build log dblog

up:
	docker compose up -d

build:
	docker compose up -d --build

log:
	docker compose logs -f app

dblog:
	docker compose logs -f db

diesel_setup:
	docker compose exec app diesel setup
