PAPERS = tiger-corpus-methodology \
         tiger-pca-dimensions \
         tiger-framework \
         tiger-fingerprinting \
         tiger-experience-prediction \
         tiger-design-gaps

.PHONY: all pdf clean

all: pdf

pdf:
	@for paper in $(PAPERS); do \
		echo "==> Building $$paper..."; \
		$(MAKE) -C publications/$$paper pdf || echo "WARN: $$paper failed"; \
	done

clean:
	@for paper in $(PAPERS); do \
		$(MAKE) -C publications/$$paper clean; \
	done
