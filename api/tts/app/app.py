from dotenv import load_dotenv
from litestar import Litestar, get, Response
from litestar.config.cors import CORSConfig

from tts.tts import tts


load_dotenv()

@get("/generate", summary="Generate WAV audio from text", sync_to_thread=True)
def generate(text: str, speaker: str, sample_rate: int, pitch: int, rate: int) -> Response:
    audio = tts.generate(text, speaker, sample_rate, pitch, rate)
    return Response(audio, media_type="audio/wav")


app = Litestar(
    [generate],
    cors_config=CORSConfig(),
)
