.PHONY: *

up:
	docker compose up --build

tunnel:
	ssh -R 10.112.114.102:9999:127.0.0.1:8000 -N -v st114@st114vm102.rtb-lab.pl
