PREFIX := /usr/local

.PHONY: install
install:
	@ mkdir -p ${PREFIX}/bin
	@ for i in ${PWD}/src/*; do \
		cp "$$i" ${PREFIX}/bin && \
		chmod 755 ${PREFIX}/bin/"$$(basename "$$i")"; \
	  done

.PHONY: uninstall
uninstall:
	@ for i in ${PWD}/src/*; do \
		rm ${PREFIX}/bin/"$$(basename "$$i")"; \
	  done
