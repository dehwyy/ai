FROM python:3.11-slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends curl bash && \
	apt-get clean && rm -rf /var/lib/apt/lists/*

FROM builder AS deps
WORKDIR /app

RUN curl -LsSf https://rye-up.com/get | RYE_INSTALL_OPTION="--yes" RYE_TOOLCHAIN=/usr/local/bin/python3 bash && \
	ln -s /root/.rye/shims/rye /usr/local/bin/rye && \
	export PATH="/root/.rye/shims:$PATH" && \
	rye pin 3.11 && \
	rye config --set-bool behavior.use-uv=true
COPY pyproject.toml .
RUN --mount=type=cache,target=/root/.cache rye sync --no-dev

FROM builder AS models-installer
COPY models models
COPY install_models.sh install_models.sh
RUN chmod +x ./install_models.sh && ./install_models.sh >&2

FROM gcr.io/distroless/python3-debian12:nonroot
WORKDIR /app
ENV PATH="/app/.venv/bin:$PATH"
ENV PYTHONPATH="/app/.venv/lib/python3.11/site-packages:$PYTHONPATH"
USER nonroot
COPY --from=models-installer /models models
COPY --from=deps /app/.venv .venv
COPY ./app app
COPY ./tts tts
CMD ["/app/.venv/bin/litestar", "run", "--host", "0.0.0.0", "--port", "8000"]
