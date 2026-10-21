/* Runs before first paint: marks JS support and decides whether the loader shows. */
(function () {
  var d = document.documentElement;
  d.classList.add("js");
  try {
    if (sessionStorage.getItem("ag-intro") || matchMedia("(prefers-reduced-motion: reduce)").matches) d.classList.add("intro-seen");
  } catch (e) {
    d.classList.add("intro-seen");
  }
})();
