# 🥔 Potato Assistant

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![GUI](https://img.shields.io/badge/GUI-Iced-blue.svg)
![AI](https://img.shields.io/badge/AI-Powered-purple.svg)

**Potato Assistant** is a modern, voice-enabled AI chat interface built with Rust. It mimics the layout of professional tools like Gemini or ChatGPT but runs natively on your desktop with high performance and low resource usage.

The goal of this project is to provide a seamless **Voice-to-Voice** and **Text-to-Text** experience, bridging the gap between local desktop environments and Large Language Models (LLMs).

---

## ✨ Features

* **🗣️ Full Voice Interaction:**
    * **Speech-to-Text (STT):** Speak naturally to the assistant.
    * **Text-to-Speech (TTS):** The AI answers back with audio.
* **💬 Modern Chat Interface:**
    * **Split View Layout:** Sidebar for conversation history (left) and main chat area (right).
    * **Streaming Responses:** Watch the AI's answer appear character by character in real-time.
* **🧠 AI Backend:** Designed to connect with major providers (OpenAI, Anthropic) or local models (Ollama).
* **🚀 Native Performance:** Built in Rust for blazing fast startup and minimal memory footprint compared to Electron apps.

## 🛠️ Tech Stack

This project leverages the best-in-class Rust crates for multimedia and async tasks:

* **GUI:** [`iced`](https://github.com/iced-rs/iced) (Model-View-Update architecture).
* **Network:** [`reqwest`](https://github.com/seanmonstar/reqwest) for handling API streams.
* **Audio Input:** [`cpal`](https://github.com/RustAudio/cpal) for low-level microphone access.
* **Audio Output:** [`rodio`](https://github.com/RustAudio/rodio) for playing AI responses.
* **Async Runtime:** [`tokio`](https://github.com/tokio-rs/tokio).

## 🚀 Prerequisites

Since this project handles Audio I/O, you need specific system libraries installed.

**On Arch Linux / Manjaro:**
```bash
sudo pacman -S alsa-lib openssl
```

**On Ubuntu / Debian:**
```bash
sudo apt install libasound2-dev libssl-dev pkg-config
```

## ⚙️ Configuration

To use the AI features, you need to set up your API keys.

1.  Create a `.env` file in the root directory:
    ```bash
    touch .env
    ```
2.  Add your API Key (Example for OpenAI or local URL):
    ```env
    AI_API_KEY=sk-your-api-key-here
    # AI_MODEL=gpt-4o
    ```

## 🛠️ Installation and Usage

1.  **Clone the project:**
    ```bash
    git clone [https://github.com/your-username/potato_assistant.git](https://github.com/your-username/potato_assistant.git)
    cd potato_assistant
    ```

2.  **Run in Release Mode:**
    For the best audio performance and UI smoothness, always run in release mode.

    ```bash
    cargo run --release
    ```

## 📂 Architecture Overview

The application manages two heavy asynchronous streams simultaneously without blocking the UI:

```
src/
├── main.rs           # UI Entry point (Iced Application).
├── audio/
│   ├── microphone.rs # Handles cpal input stream.
│   └── speaker.rs    # Handles rodio output queue.
├── api/
│   └── client.rs     # Manages HTTP requests and SSE (Server-Sent Events) streaming.
└── ui/
    ├── chat.rs       # The main chat view component.
    └── sidebar.rs    # The history sidebar component.
```

## 🤝 Contributing

We welcome "Potato" enthusiasts! If you want to improve the voice detection or add support for more local AI models:

1.  Fork the project.
2.  Create your feature branch (`git checkout -b feature/BetterVoice`).
3.  Commit your changes.
4.  Push to the branch.
5.  Open a Pull Request.

---

*Because even a Potato can be smart with enough Rust.* 🥔
