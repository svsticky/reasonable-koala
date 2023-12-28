all: help
.SILENT:

.PHONY: help
help:
	echo "Wilford OAuth2 Server"
	echo "Commands: "
	echo "- upload-all		: Build and upload all Docker images"
	echo "- upload-server 	: Build and upload the server Docker image"
	echo "- upload-docs		: Build and upload the docs Docker image"
	echo "- upload-ui	 	: Build and upload the ui Docker image"

.PHONY: upload-all
upload-all: upload-server upload-docs upload-ui

.PHONY: upload-server
upload-server: build-server
	docker push registry.mrfriendly.uk/wilford-server

.PHONY: upload-docs
upload-docs: build-docs
	docker push registry.mrfriendly.uk/wilford-docs

.PHONY: upload-ui
upload-ui: build-ui
	docker push registry.mrfriendly.uk/wilford-ui

.PHONY: build-server
build-server:
	docker build -t registry.mrfriendly.uk/wilford-server server/

.PHONY: build-docs
build-docs:
	docker build -t registry.mrfriendly.uk/wilford-docs docs/

.PHONY: build-ui
build-ui:
	docker build -t registry.mrfriendly.uk/wilford-ui ui/