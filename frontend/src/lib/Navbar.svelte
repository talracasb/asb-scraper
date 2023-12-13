<nav>
	<div class="nav-items">
		<div class="split left">
			<a class="link logo" href="/">betterasb</a>
			<div class="nav-links">
				<a class="link" href="/grades">Grades</a>
				<a class="link" href="/schedule">Schedule</a>
			</div>
		</div>
		<div class="split right">
			<a class="github" href="https://github.com/talracasb/asb-scraper">
				<img src="/svg/github.svg" alt="Github Logo" />
			</a>
		</div>
	</div>
</nav>

<style lang="less">
	//@import (css) url('https://fonts.googleapis.com/css2?family=Comic+Neue:wght@400;700&family=Fira+Code:wght@500&family=New+Rocker&display=swap');

	.nav-items {
		display: flex;
		width: auto;
		height: 100%;
		justify-content: space-between;
		align-items: center;
		padding-left: 3rem;
		padding-right: 3rem;
	}

	.nav-links {
		display: flex;
		width: 100%;
		gap: 1rem;
	}

	.split {
		display: flex;
		width: 100%;
		gap: 2rem;
		align-items: center;
	}

	.right {
		flex-direction: row-reverse;
	}

	.github {
		height: 2rem;

		img {
			height: 100%;
		}
	}

	.link {
		font-size: 1.1rem;
		text-decoration: none;
		color: var(--fg);
		padding: 0;
		margin: 0;
		text-align: center;
	}

	.logo {
		font-family:
			Consolas,
			Monaco,
			Andale Mono,
			Ubuntu Mono,
			monospace;
		font-size: 1.5rem;
	}

	nav {
		position: fixed;
		width: 100%;
		background-color: var(--bg-2);
		height: 5rem;
	}
</style>
