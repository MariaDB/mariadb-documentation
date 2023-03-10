import logging
logging.basicConfig(
    format="[%(levelname)s] %(asctime)s %(message)s",
    datefmt="%H:%M:%S",
    level=logging.INFO,
)
log = logging.getLogger()
log.setLevel(logging.INFO)
