<script>
	import { Router, Route } from "svelte-routing";
	import { websocket } from "./store/websocketStore";
	import WebSocketClient from "./lib/WebSocketClient";
	import Home from "./routes/Home.svelte";
	import ExtractAnimationData from "./routes/ExtractAnimationData.svelte";

	let socket = new WebSocketClient();
	// When you write $websocket, you're essentially saying, "Get the current value from the store named websocket."
	$websocket = socket; // Assign the instance to the store

	// Connect to the websocket
	socket.connect("ws://localhost:3333/ws");

	// Send messages or receive events
	// socket.sendMessage("Hello, server!");
	// socket.on('message', (event) => {
	// 	console.log(event.data);
	// });
</script>

<Router>
	<div>
		<Route path="/" component={Home} />
		<Route path="/extract" component={ExtractAnimationData} />
	</div>
</Router>
