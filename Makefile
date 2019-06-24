JEKYLL=bundle exec jekyll
SHELL=bash

$(eval CONFIG= \
	$(shell find config -maxdepth 1 -type f -name '*.yml' | \
		  sort -g | awk -vORS=, ' {print $1} ')_config.yml)

$(eval OPTS= \
	--config "$(CONFIG)")

##
# Targets
help:
	@echo 'Makefile for Jekyll site'
	@echo ''
	@echo 'Usage:'
	@echo 'make init            Initialize directory'
	@echo 'make html            Generate the web site'
	@echo 'make clean           Clean up generated site'

.PHONY: init
init:
	bundle install --path .vendor/bundle

.PHONY: html
html:
	$(JEKYLL) build $(OPTS)

.PHONY: serve
serve:
	$(JEKYLL) serve -w $(OPTS)


.PHONY: clean
clean:
	$(JEKYLL) clean

