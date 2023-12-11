<nav>
    <div class="nav-items">
        <a class="logo" href="/">betterasb</a>
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

    .logo {
        font-family: Consolas, Monaco, Andale Mono, Ubuntu Mono, monospace;
        font-size: 1.5rem;
        text-decoration: none;
        color: var(--fg);
        padding: 0;
        margin: 0;
        text-align: center;
    }

    nav {
        position: fixed;
        width: 100%;
        background-color: var(--bg-2);
        height: 5rem;
    }
</style>