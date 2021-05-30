.PHONY: clean debug release run

pydir = python
libname = hexnom

release:
	cargo build --release
	@cp target/release/lib$(libname).so $(pydir)/$(module_name).so

debug:
	cargo build
	@cp target/debug/lib$(libname).so $(pydir)/$(module_name).so

clean:
	cargo clean
	rm $(pydir)/*.so

run: release
	python3 $(pydir)/main.py
