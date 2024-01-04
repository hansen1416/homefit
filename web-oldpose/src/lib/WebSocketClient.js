export default class WebSocketClient {
	constructor() {
		this.socket = null;
		this.onMessage = () => {}; // Define empty default event handlers
		this.onError = () => {};
		this.onClose = () => {};
	}

	connect(url) {
		this.socket = new WebSocket(url);

		this.socket.onopen = () => {
			console.log("WebSocket connected!");
		};

		this.socket.onmessage = (event) => {
			this.onMessage(event.data); // Call user-defined onMessage handler
		};

		this.socket.onerror = (error) => {
			console.error("WebSocket error:", error);
			this.onError(error); // Call user-defined onError handler
		};

		this.socket.onclose = (event) => {
			console.log("WebSocket closed:", event.code, event.reason);
			this.onClose(event); // Call user-defined onClose handler
		};
	}

	sendMessage(message) {
		if (this.socket.readyState === WebSocket.OPEN) {
			this.socket.send(message);
		} else {
			console.error("WebSocket is not open, cannot send message.");
		}
	}

	close() {
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
	}
}
