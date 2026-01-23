if (window) {
    const mediaQuery = "(prefers-color-scheme: dark)";
    const mqList = window.matchMedia(mediaQuery);
    const document = window.document;
    const htmlElement = document.documentElement;
    htmlElement.setAttribute(
        "data-theme",
        mqList.matches ? "dark" : "light"
    );
    const handleThemeChange = (event) => {
        htmlElement.setAttribute(
            "data-theme",
            event.matches ? "dark" : "light"
        );
    };
    mqList.addEventListener("change", handleThemeChange);
    window.addEventListener('beforeunload', () => {
        mqList.removeEventListener("change", handleThemeChange);
    });
}