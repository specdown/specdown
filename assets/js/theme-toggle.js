(function () {
  var root = document.documentElement;
  var STORAGE_KEY = "specdown-theme";

  function currentTheme() {
    var stored = localStorage.getItem(STORAGE_KEY);
    if (stored) return stored;
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }

  function setTheme(theme) {
    root.setAttribute("data-theme", theme);
    localStorage.setItem(STORAGE_KEY, theme);
  }

  document.addEventListener("DOMContentLoaded", function () {
    var toggle = document.querySelector(".js-theme-toggle");
    if (toggle) {
      toggle.addEventListener("click", function () {
        setTheme(currentTheme() === "dark" ? "light" : "dark");
      });
    }

    var sidebarToggle = document.querySelector(".js-sidebar-toggle");
    var backdrop = document.querySelector(".js-docs-backdrop");
    if (sidebarToggle) {
      sidebarToggle.addEventListener("click", function () {
        document.body.classList.toggle("sidebar-open");
      });
    }
    if (backdrop) {
      backdrop.addEventListener("click", function () {
        document.body.classList.remove("sidebar-open");
      });
    }
  });
})();
