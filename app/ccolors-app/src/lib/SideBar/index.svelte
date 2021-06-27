<script></script>

<div class="h-full w-60 flex-grow-0 flex-shrink-0 flex flex-col items-start pt-10" id="sidebar">
	<button class="w-4/5 h-10 mx-auto bg-white rounded-lg border border-black font-bold rainbow">
		Random Color
	</button>
</div>

<style>
	#sidebar {
		background-color: #d6d8d8;
		box-shadow: 2px 0px 6px rgba(0, 0, 0, 0.2);
	}

	/* .rainbow {
		background-color: white;
	} */

	.rainbow:hover {
		/* background-color: transparent; */
		background: linear-gradient(
			45deg,
			#ff000060,
			#ff730060,
			#fffb0060,
			#48ff0060,
			#00ffd560,
			#002bff60,
			#7a00ff60,
			#ff00c860,
			#ff000060
		);
		background-size: 350%;
		animation: glowing 20s linear infinite;
		border-radius: 10px;
	}

	@keyframes glowing {
		0% {
			background-position: 0 0;
		}
		50% {
			background-position: 400% 0;
		}
		100% {
			background-position: 0 0;
		}
	}
</style>
