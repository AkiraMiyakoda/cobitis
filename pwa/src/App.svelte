<script lang="ts">
</script>

<main>
    <div class="fill bg"></div>
</main>

<style lang="scss">
    @font-face {
        font-display: swap;
        font-family: "Titillium Web";
        font-style: normal;
        font-weight: 400;
        src:
            url("assets/fonts/titillium-web-v17-latin-regular.woff2") format("woff2"),
            url("assets/fonts/titillium-web-v17-latin-regular.ttf") format("truetype");
    }

    @font-face {
        font-display: swap;
        font-family: "Titillium Web";
        font-style: normal;
        font-weight: 600;
        src:
            url("assets/fonts/titillium-web-v17-latin-600.woff2") format("woff2"),
            url("assets/fonts/titillium-web-v17-latin-600.ttf") format("truetype");
    }

    @font-face {
        font-display: swap;
        font-family: "Titillium Web";
        font-style: normal;
        font-weight: 700;
        src:
            url("assets/fonts/titillium-web-v17-latin-700.woff2") format("woff2"),
            url("assets/fonts/titillium-web-v17-latin-700.ttf") format("truetype");
    }

    // MVP.css customization
    :root {
        --active-brightness: 0.85;
        --border-radius: 5px;
        --box-shadow: 2px 2px 10px;
        --color-accent: #ffa69e;
        --color-bg: #5e6472;
        --color-bg-secondary: #aed9e0;
        --color-link: #118bee;
        --color-secondary: #920de9;
        --color-secondary-accent: #920de90b;
        --color-shadow: #f4f4f4;
        --color-table: #118bee;
        --color-text: #faf3dd;
        --color-text-secondary: #b8f2e6;
        --font-family: "Titillium Web", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu,
            Cantarell, "Helvetica Neue", sans-serif;
        --hover-brightness: 1.2;
        --justify-important: center;
        --justify-normal: left;
        --line-height: 1.5;
        --width-card: 285px;
        --width-card-medium: 460px;
        --width-card-wide: 800px;
        --width-content: 1080px;
    }

    .fill {
        position: absolute;
        left: 0;
        top: 0;
        width: 100vw;
        height: 100vh;
        overflow: hidden;
    }

    .bg {
        background-image: url("assets/images/bg.png");
        background-repeat: repeat;
    }
</style>
