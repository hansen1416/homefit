<script>
	import { Router, Route } from "svelte-routing";
	import WebSocketClient from "./lib/WebSocketClient";
	import Scene from "./components/Scene.svelte";
	import TextBubble from "./components/TextBubble.svelte";
	import ExtractAnimationData from "./routes/ExtractAnimationData.svelte";
	import Home from "./routes/Home.svelte";
	import Gym from "./routes/Gym.svelte";

	import websocket_state from "./store/websocketStore";
	import animation_data from "./store/animationDataStore";
	import animation_queue from "./store/animationQueueStore";

	let socket = new WebSocketClient();
	// When you write $websocket, you're essentially saying, "Get the current value from the store named websocket."

	// websocket.set(socket); // Assign the instance to the store
	// Connect to the websocket
	socket.connect("ws://localhost:3333/ws");

	socket.onConnect = (ws_state) => {
		websocket_state.set(ws_state);
	};

	socket.onMessage = (msg) => {
		// check if msg is an ArrayBuffer(0)
		if (msg instanceof ArrayBuffer && msg.byteLength === 0) {
			console.log("received heartbeat message");
			return;
		}

		if (typeof msg !== "string") {
			console.log("websocket message received non-string message", msg);
			return;
		}

		// first split the message
		const [redis_key, data] = msg.split("::");

		const [category, name] = redis_key.split(":");

		if (category === "am") {
			// 'am' is animation data for a single animation
			console.log("received animation data", name);

			animation_data.update((old_data) => {
				return { ...old_data, [name]: data };
			});
		} else if (category === "amq") {
			// 'amq' is animation queue, a list of animation metadata

			// update animation_queue
			animation_queue.update((old_queue) => {
				return [...old_queue, ...JSON.parse(data)];
			});
		} else {
			console.log("received unknown message", msg);
		}
	};
</script>

<Router>
	<div>
		<Scene />
		<TextBubble />

		<Route path="/" component={Home} />
		<Route path="/extract" component={ExtractAnimationData} />
		<Route path="/gym/:id" let:params>
			<Gym id={params.id} />
		</Route>
	</div>
</Router>
