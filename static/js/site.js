/* Alfa Global Group: interactions and motion. No dependencies. */
(function () {
  "use strict";

  var root = document.documentElement;
  var reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  var finePointer = window.matchMedia("(hover: hover) and (pointer: fine)").matches;
  var $ = function (s, el) { return (el || document).querySelector(s); };
  var $$ = function (s, el) { return Array.prototype.slice.call((el || document).querySelectorAll(s)); };

  /* --------------------------------------------------------------- Loader */

  // The preview site's preloader: shown once per browser session.
  var afterIntro = [];
  var introPlaying = false;

  function whenIntroDone(fn) { if (introPlaying) afterIntro.push(fn); else fn(); }

  function setupIntro() {
    var loader = $("[data-intro]");
    if (!loader || root.classList.contains("intro-seen")) return;
    introPlaying = true;
    try { sessionStorage.setItem("ag-intro", "1"); } catch (e) { /* private mode */ }
    document.body.classList.add("is-locked");

    var box = $("[data-particles]", loader);
    for (var i = 0; box && i < 40; i++) {
      var p = document.createElement("span");
      var size = 1 + Math.random() * 2;
      p.style.cssText = "left:" + (Math.random() * 100) + "%;top:" + (Math.random() * 100) + "%;width:" + size + "px;height:" + size +
        "px;background:" + (i % 2 ? "#2e7d32" : "#00e5ff") + ";--t:" + (2 + Math.random() * 3).toFixed(2) + "s;--dl:" + (Math.random() * 3).toFixed(2) + "s";
      box.appendChild(p);
    }

    var fill = $("[data-progress]", loader);
    var label = $("[data-percent]", loader);
    var progress = 0;
    var loaded = document.readyState === "complete";
    window.addEventListener("load", function () { loaded = true; });

    var finished = false;
    function finish() {
      if (finished) return;
      finished = true;
      clearInterval(timer);
      loader.classList.add("is-done");
      document.body.classList.remove("is-locked");
      introPlaying = false;
      afterIntro.splice(0).forEach(function (fn) { fn(); });
      setTimeout(function () { loader.remove(); }, 800);
    }
    var timer = setInterval(function () {
      // Hold at 90% until the page (images, fonts) has finished loading.
      progress = Math.min(loaded ? 100 : 90, progress + Math.random() * 8);
      if (fill) fill.style.width = progress + "%";
      if (label) label.textContent = Math.round(progress) + "%";
      if (progress >= 100) { clearInterval(timer); setTimeout(finish, 500); }
    }, 60);
    setTimeout(finish, 8000); // never keep visitors waiting on a slow asset
    document.addEventListener("keydown", function onKey(e) {
      if (e.key === "Escape") { finish(); document.removeEventListener("keydown", onKey); }
    });
  }

  /* --------------------------------------------------------------- Header */

  function setupHeader() {
    var nav = $("[data-nav]");
    var bar = $(".progress");
    var top = $("[data-top]");
    var last = window.scrollY;
    var ticking = false;

    function update() {
      var y = window.scrollY;
      var max = document.documentElement.scrollHeight - window.innerHeight;
      if (nav) {
        nav.classList.toggle("is-scrolled", y > 24);
        var menuOpen = document.body.classList.contains("menu-open");
        nav.classList.toggle("is-hidden", !menuOpen && y > 480 && y > last + 4);
        if (y < last - 4 || y < 480) nav.classList.remove("is-hidden");
      }
      if (bar) bar.style.setProperty("--p", max > 0 ? (y / max).toFixed(4) : 0);
      if (top) top.classList.toggle("is-visible", y > 900);
      last = y;
      ticking = false;
    }
    window.addEventListener("scroll", function () {
      if (!ticking) { ticking = true; requestAnimationFrame(update); }
    }, { passive: true });
    update();

    if (top) top.addEventListener("click", function () { window.scrollTo({ top: 0, behavior: reduceMotion ? "auto" : "smooth" }); });

    // Mobile menu
    var toggle = $("[data-menu-toggle]");
    var menu = $("[data-menu]");
    if (!toggle || !menu) return;
    function setOpen(open) {
      toggle.setAttribute("aria-expanded", String(open));
      toggle.setAttribute("aria-label", open ? "Close menu" : "Open menu");
      menu.classList.toggle("is-open", open);
      document.body.classList.toggle("is-locked", open);
      document.body.classList.toggle("menu-open", open);
      if (open) { var first = $("a", menu); if (first) setTimeout(function () { first.focus({ preventScroll: true }); }, 300); }
    }
    toggle.addEventListener("click", function () { setOpen(toggle.getAttribute("aria-expanded") !== "true"); });
    document.addEventListener("keydown", function (e) {
      if (e.key === "Escape" && menu.classList.contains("is-open")) { setOpen(false); toggle.focus(); }
    });
    window.addEventListener("resize", function () { if (window.innerWidth >= 1140) setOpen(false); });
  }

  /* --------------------------------------------------------------- Reveal */

  function setupReveal(root) {
    var items = $$(".reveal:not(.is-in)", root);
    if (!("IntersectionObserver" in window) || reduceMotion) {
      items.forEach(function (el) { el.classList.add("is-in"); });
      return;
    }
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) { entry.target.classList.add("is-in"); io.unobserve(entry.target); }
      });
    }, { rootMargin: "0px 0px -8% 0px", threshold: 0.08 });
    items.forEach(function (el) { io.observe(el); });
  }

  /* ------------------------------------------------------------- Counters */

  function setupCounters() {
    var els = $$("[data-count]");
    if (!els.length) return;
    function run(el) {
      var target = parseFloat(el.getAttribute("data-count"));
      var suffix = el.getAttribute("data-suffix") || "";
      if (reduceMotion) { el.textContent = target + suffix; return; }
      var start = null;
      var dur = 1800;
      function step(ts) {
        if (start === null) start = ts;
        var t = Math.min(1, (ts - start) / dur);
        var eased = 1 - Math.pow(1 - t, 4);
        el.textContent = Math.round(target * eased) + suffix;
        if (t < 1) requestAnimationFrame(step);
      }
      el.textContent = "0" + suffix;
      requestAnimationFrame(step);
    }
    if (!("IntersectionObserver" in window)) return;
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (!entry.isIntersecting) return;
        io.unobserve(entry.target);
        var inHero = entry.target.closest("[data-hero]");
        if (inHero) whenIntroDone(function () { setTimeout(function () { run(entry.target); }, 700); });
        else run(entry.target);
      });
    }, { threshold: 0.5 });
    els.forEach(function (el) { io.observe(el); });
  }

  /* ------------------------------------------- Tilt, spotlight, magnetic */

  function setupPointerFx(root) {
    if (!finePointer || reduceMotion) return;

    $$(".card, .adv", root).forEach(function (card) {
      var tilt = card.hasAttribute("data-tilt");
      var raf = 0;
      card.addEventListener("pointermove", function (e) {
        var r = card.getBoundingClientRect();
        var x = (e.clientX - r.left) / r.width;
        var y = (e.clientY - r.top) / r.height;
        cancelAnimationFrame(raf);
        raf = requestAnimationFrame(function () {
          card.style.setProperty("--mx", (x * 100).toFixed(1) + "%");
          card.style.setProperty("--my", (y * 100).toFixed(1) + "%");
          if (tilt) {
            card.style.setProperty("--rx", ((0.5 - y) * 7).toFixed(2) + "deg");
            card.style.setProperty("--ry", ((x - 0.5) * 9).toFixed(2) + "deg");
          }
        });
      });
      card.addEventListener("pointerleave", function () {
        cancelAnimationFrame(raf);
        card.style.removeProperty("--rx");
        card.style.removeProperty("--ry");
      });
    });

    $$("[data-magnetic]", root).forEach(function (btn) {
      btn.addEventListener("pointermove", function (e) {
        var r = btn.getBoundingClientRect();
        var dx = e.clientX - (r.left + r.width / 2);
        var dy = e.clientY - (r.top + r.height / 2);
        btn.style.transform = "translate(" + (dx * 0.18).toFixed(1) + "px," + (dy * 0.28).toFixed(1) + "px)";
      });
      btn.addEventListener("pointerleave", function () { btn.style.transform = ""; });
    });
  }

  /* ----------------------------------------------------------- Focus tabs */

  function setupFocus() {
    var wrap = $("[data-focus]");
    if (!wrap) return;
    var tabs = $$("[role=tab]", wrap);
    var panels = tabs.map(function (t) { return document.getElementById(t.getAttribute("aria-controls")); });
    var current = 0;
    var timer = 0;
    var inView = false;
    var hovering = false;
    var PERIOD = 7000;

    function select(i, focus) {
      current = (i + tabs.length) % tabs.length;
      tabs.forEach(function (t, j) {
        var on = j === current;
        t.setAttribute("aria-selected", String(on));
        t.tabIndex = on ? 0 : -1;
        panels[j].hidden = !on;
      });
      var p = panels[current];
      p.classList.remove("is-entering");
      void p.offsetWidth;
      p.classList.add("is-entering");
      if (focus) tabs[current].focus();
      schedule();
    }
    function schedule() {
      clearTimeout(timer);
      if (reduceMotion || !inView || hovering) return;
      timer = setTimeout(function () { select(current + 1, false); }, PERIOD);
    }
    tabs.forEach(function (t, i) {
      t.addEventListener("click", function () { select(i, false); });
      t.addEventListener("keydown", function (e) {
        if (e.key === "ArrowDown" || e.key === "ArrowRight") { e.preventDefault(); select(i + 1, true); }
        if (e.key === "ArrowUp" || e.key === "ArrowLeft") { e.preventDefault(); select(i - 1, true); }
      });
    });
    wrap.addEventListener("pointerenter", function () { hovering = true; wrap.classList.add("is-paused"); clearTimeout(timer); });
    wrap.addEventListener("pointerleave", function () { hovering = false; wrap.classList.remove("is-paused"); select(current, false); });
    wrap.style.setProperty("--focus-ms", PERIOD + "ms");
    if (reduceMotion) wrap.classList.add("is-paused");
    if ("IntersectionObserver" in window) {
      new IntersectionObserver(function (entries) {
        inView = entries[0].isIntersecting;
        if (inView) select(current, false); else clearTimeout(timer);
      }, { threshold: 0.35 }).observe(wrap);
    }
  }

  /* -------------------------------------------------------- Team profiles */

  // Delegated, so profiles keep working after a live refresh replaces the grid.
  function setupProfiles() {
    function open(id) {
      var dlg = document.getElementById(id);
      if (!dlg || typeof dlg.showModal !== "function") return;
      dlg.showModal();
      document.body.classList.add("is-locked");
    }
    document.addEventListener("click", function (e) {
      var btn = e.target.closest && e.target.closest("[data-profile]");
      if (btn) { open(btn.getAttribute("data-profile")); return; }
      var dlg = e.target.closest && e.target.closest("dialog.profile");
      if (!dlg) return;
      if (e.target === dlg || e.target.closest("[data-close]")) dlg.close();
    });
    document.addEventListener("close", function (e) {
      if (e.target.matches && e.target.matches("dialog.profile")) document.body.classList.remove("is-locked");
    }, true);
    var m = /^#member-(\d+)$/.exec(location.hash);
    if (m) {
      var card = document.getElementById("member-" + m[1]);
      if (card) {
        card.classList.add("is-in");
        setTimeout(function () { open("profile-" + m[1]); }, 450);
      }
    }
  }


  /* ---------------------------------------------------------------- Forms */

  function setupForms() {
    $$("[data-form]").forEach(function (form) {
      form.addEventListener("submit", function () {
        var btn = $("button[type=submit]", form);
        if (btn) {
          btn.classList.add("is-loading");
          btn.innerHTML = 'Sending <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M12 3a9 9 0 1 0 9 9"/></svg>';
        }
      });
    });
  }

  /* ---------------------------------------------------------------- Globe */

  // Rough continent outlines as [lon, lat] rings; enough to read as a world.
  var LAND = [
    [[-168,65],[-140,70],[-95,72],[-80,63],[-60,55],[-55,47],[-70,43],[-76,35],[-81,25],[-97,26],[-105,20],[-95,16],[-83,9],[-78,8],[-88,15],[-105,22],[-115,30],[-125,40],[-125,49],[-135,58],[-150,60],[-165,60]],
    [[-55,60],[-45,60],[-20,70],[-20,80],[-60,82],[-72,77],[-55,68]],
    [[-80,9],[-60,11],[-50,0],[-35,-6],[-40,-22],[-48,-27],[-58,-38],[-65,-55],[-73,-50],[-71,-30],[-77,-14],[-81,-5]],
    [[-10,36],[-9,43],[-2,48],[-5,58],[5,62],[15,70],[30,71],[40,66],[45,55],[40,45],[28,41],[22,37],[15,38],[10,44],[3,43]],
    [[-6,50],[2,51],[-2,56],[-5,58],[-6,55]],
    [[-17,21],[-10,30],[0,36],[10,37],[20,32],[32,31],[35,28],[43,12],[51,12],[40,-2],[40,-15],[33,-26],[20,-35],[15,-27],[12,-15],[10,-2],[8,4],[-8,5],[-17,14]],
    [[40,66],[60,70],[80,73],[105,78],[140,72],[170,70],[178,65],[160,58],[140,52],[135,43],[122,40],[121,30],[110,20],[105,10],[100,13],[98,8],[92,22],[88,22],[80,15],[77,8],[72,20],[66,25],[57,25],[52,30],[48,30],[36,36],[28,41],[40,45],[45,55]],
    [[35,28],[43,12],[52,15],[60,22],[57,26],[50,26],[48,30],[36,35]],
    [[95,5],[105,-6],[120,-9],[140,-8],[135,-3],[118,2],[110,2],[100,3]],
    [[114,-22],[122,-17],[130,-12],[137,-12],[142,-11],[146,-19],[153,-27],[150,-37],[141,-38],[131,-31],[115,-34]],
    [[130,31],[135,34],[141,38],[142,45],[140,41],[135,35]]
  ];
  var HUB = { lat: 13.08, lon: 80.27 };
  var CITIES = [
    { lat: 25.2, lon: 55.27 }, { lat: 1.35, lon: 103.8 }, { lat: 51.5, lon: -0.12 },
    { lat: 40.7, lon: -74 }, { lat: 35.7, lon: 139.7 }, { lat: 50.1, lon: 8.7 }, { lat: -33.9, lon: 151.2 }
  ];

  function inRing(lon, lat, ring) {
    var inside = false;
    for (var i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      var xi = ring[i][0], yi = ring[i][1], xj = ring[j][0], yj = ring[j][1];
      if ((yi > lat) !== (yj > lat) && lon < (xj - xi) * (lat - yi) / (yj - yi) + xi) inside = !inside;
    }
    return inside;
  }
  function isLand(lon, lat) {
    for (var i = 0; i < LAND.length; i++) if (inRing(lon, lat, LAND[i])) return true;
    return false;
  }
  function vec(lat, lon) {
    var p = lat * Math.PI / 180, l = lon * Math.PI / 180;
    return [Math.cos(p) * Math.sin(l), Math.sin(p), Math.cos(p) * Math.cos(l)];
  }

  function Globe(canvas) {
    var ctx = canvas.getContext("2d");
    var small = window.innerWidth < 700;
    var N = small ? 2600 : 5200;
    var pts = [];
    var golden = Math.PI * (3 - Math.sqrt(5));
    for (var i = 0; i < N; i++) {
      var y = 1 - (i / (N - 1)) * 2;
      var r = Math.sqrt(1 - y * y);
      var th = golden * i;
      var lat = Math.asin(y) * 180 / Math.PI;
      var lon = ((Math.atan2(Math.sin(th) * r, Math.cos(th) * r) * 180 / Math.PI) + 540) % 360 - 180;
      pts.push({ v: [Math.cos(th) * r, y, Math.sin(th) * r], land: isLand(lon, lat) });
    }
    // store unit vectors in the same convention as vec()
    pts.forEach(function (p) { var v = p.v; p.v = [v[2], v[1], v[0]]; });
    var hub = vec(HUB.lat, HUB.lon);
    var arcs = CITIES.map(function (c, i) { return { to: vec(c.lat, c.lon), phase: i / CITIES.length }; });

    var w = 0, h = 0, dpr = 1, R = 0, cx = 0, cy = 0;
    var rot = -HUB.lon * Math.PI / 180 + 0.25;
    var tilt = -0.28;
    var targetTiltX = 0, targetTiltY = 0, tx = 0, ty = 0;
    var running = false, visible = false, last = 0, t0 = performance.now();

    function resize() {
      var rect = canvas.getBoundingClientRect();
      dpr = Math.min(window.devicePixelRatio || 1, 2);
      w = Math.max(1, rect.width); h = Math.max(1, rect.height);
      canvas.width = Math.round(w * dpr); canvas.height = Math.round(h * dpr);
      R = Math.min(w, h) * 0.4; cx = w / 2; cy = h / 2;
      draw(performance.now());
    }

    function project(v, out) {
      var ry = rot + ty;
      var cr = Math.cos(ry), sr = Math.sin(ry);
      var x = v[0] * cr + v[2] * sr;
      var z = -v[0] * sr + v[2] * cr;
      var y = v[1];
      var tl = tilt + tx;
      var ct = Math.cos(tl), st = Math.sin(tl);
      var y2 = y * ct - z * st;
      var z2 = y * st + z * ct;
      out[0] = cx + x * R; out[1] = cy - y2 * R; out[2] = z2;
      return out;
    }

    function slerp(a, b, t, lift) {
      var dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
      var om = Math.acos(Math.max(-1, Math.min(1, dot)));
      var so = Math.sin(om) || 1;
      var k1 = Math.sin((1 - t) * om) / so, k2 = Math.sin(t * om) / so;
      var s = 1 + lift * Math.sin(Math.PI * t);
      return [(a[0] * k1 + b[0] * k2) * s, (a[1] * k1 + b[1] * k2) * s, (a[2] * k1 + b[2] * k2) * s];
    }

    var tmp = [0, 0, 0];
    function draw(now) {
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, w, h);

      // atmosphere
      var g = ctx.createRadialGradient(cx, cy, R * 0.85, cx, cy, R * 1.35);
      g.addColorStop(0, "rgba(79,179,232,0.22)");
      g.addColorStop(1, "rgba(79,179,232,0)");
      ctx.fillStyle = g;
      ctx.beginPath(); ctx.arc(cx, cy, R * 1.35, 0, Math.PI * 2); ctx.fill();
      var body = ctx.createRadialGradient(cx - R * 0.35, cy - R * 0.4, R * 0.1, cx, cy, R);
      body.addColorStop(0, "rgba(21,144,212,0.35)");
      body.addColorStop(1, "rgba(5,37,64,0.15)");
      ctx.fillStyle = body;
      ctx.beginPath(); ctx.arc(cx, cy, R, 0, Math.PI * 2); ctx.fill();

      var dot = Math.max(1, R / 210);
      for (var i = 0; i < pts.length; i++) {
        var p = pts[i];
        project(p.v, tmp);
        var z = tmp[2];
        if (z < -0.05) { if (!p.land) continue; ctx.fillStyle = "rgba(143,208,243,0.07)"; ctx.fillRect(tmp[0], tmp[1], dot, dot); continue; }
        if (p.land) {
          ctx.fillStyle = "rgba(" + Math.round(143 + 60 * z) + "," + Math.round(208 + 30 * z) + ",243," + (0.35 + 0.65 * z).toFixed(3) + ")";
          var s = dot * (1.1 + 0.9 * z);
          ctx.fillRect(tmp[0] - s / 2, tmp[1] - s / 2, s, s);
        } else {
          ctx.fillStyle = "rgba(143,208,243," + (0.05 + 0.12 * z).toFixed(3) + ")";
          ctx.fillRect(tmp[0], tmp[1], dot * 0.9, dot * 0.9);
        }
      }

      // arcs from Chennai
      var time = (now - t0) / 1000;
      ctx.lineCap = "round";
      arcs.forEach(function (a) {
        var head = ((time * 0.28 + a.phase) % 1.4);
        var steps = 40;
        var prev = null;
        for (var k = 0; k <= steps; k++) {
          var t = k / steps;
          var v = slerp(hub, a.to, t, 0.22);
          var q = project(v, [0, 0, 0]);
          if (prev && q[2] > -0.1 && prev[2] > -0.1) {
            var near = Math.max(0, 1 - Math.abs(t - head) * 4);
            ctx.strokeStyle = "rgba(155,226,159," + (0.12 + 0.75 * near).toFixed(3) + ")";
            ctx.lineWidth = (0.8 + 1.6 * near) * Math.max(1, R / 220);
            ctx.beginPath(); ctx.moveTo(prev[0], prev[1]); ctx.lineTo(q[0], q[1]); ctx.stroke();
          }
          prev = q;
        }
        var end = project(a.to, [0, 0, 0]);
        if (end[2] > 0) {
          ctx.fillStyle = "rgba(220,240,251,0.95)";
          ctx.beginPath(); ctx.arc(end[0], end[1], 2.4 * Math.max(1, R / 220), 0, Math.PI * 2); ctx.fill();
        }
      });

      var hp = project(hub, [0, 0, 0]);
      if (hp[2] > 0) {
        var pulse = (time % 2) / 2;
        ctx.strokeStyle = "rgba(92,194,101," + (0.8 * (1 - pulse)).toFixed(3) + ")";
        ctx.lineWidth = 2;
        ctx.beginPath(); ctx.arc(hp[0], hp[1], 4 + pulse * 22, 0, Math.PI * 2); ctx.stroke();
        ctx.fillStyle = "#5cc265";
        ctx.beginPath(); ctx.arc(hp[0], hp[1], 4.5, 0, Math.PI * 2); ctx.fill();
      }
    }

    function frame(now) {
      if (!running) return;
      var dt = Math.min(64, now - (last || now));
      last = now;
      rot += dt * 0.00009;
      tx += (targetTiltX - tx) * 0.05;
      ty += (targetTiltY - ty) * 0.05;
      draw(now);
      requestAnimationFrame(frame);
    }
    function start() { if (running || reduceMotion) return; running = true; last = 0; requestAnimationFrame(frame); }
    function stop() { running = false; }

    if (finePointer && !reduceMotion) {
      window.addEventListener("pointermove", function (e) {
        targetTiltX = ((e.clientY / window.innerHeight) - 0.5) * 0.25;
        targetTiltY = ((e.clientX / window.innerWidth) - 0.5) * 0.5;
      }, { passive: true });
    }
    window.addEventListener("resize", resize);
    document.addEventListener("visibilitychange", function () { if (document.hidden) stop(); else if (visible) start(); });
    if ("IntersectionObserver" in window) {
      new IntersectionObserver(function (entries) {
        visible = entries[0].isIntersecting;
        if (visible) start(); else stop();
      }).observe(canvas);
    } else { visible = true; start(); }
    resize();
  }

  /* ----------------------------------------------------------------- Boot */

  /* ---------------------------------------------------------- Company logos */

  // Two company logos are hosted on the old site. If one cannot load, show
  // the Group badge rather than a broken image.
  function setupLogos(root) {
    $$("img[data-logo]", root).forEach(function (img) {
      var swap = function () {
        if (img.dataset.fallback) return;
        img.dataset.fallback = "1";
        img.src = "/static/img/icon-192.png";
      };
      if (img.complete && img.naturalWidth === 0 && img.src) swap();
      else img.addEventListener("error", swap);
    });
  }

  /* ----------------------------------------------------------- Live data */

  // Team and Services come live from NocoDB. Open pages check every 30 s and
  // swap in the new list when something changed, keeping the scroll position.
  function setupLive() {
    $$("[data-live]").forEach(function (box) {
      var url = box.getAttribute("data-live");
      var last = null;
      var busy = false;
      function tick() {
        if (busy || document.hidden || document.querySelector("dialog[open]")) return;
        busy = true;
        fetch(url, { cache: "no-store", headers: { "x-requested-with": "live" } })
          .then(function (r) { return r.ok ? r.text() : null; })
          .then(function (html) {
            if (html == null) return;
            if (last === null) { last = html; return; }
            if (html === last) return;
            last = html;
            box.innerHTML = html;
            $$(".reveal", box).forEach(function (el) { el.classList.add("is-in"); });
            setupLogos(box);
            setupPointerFx(box);
          })
          .catch(function () {})
          .then(function () { busy = false; });
      }
      tick();
      setInterval(tick, 30000);
      document.addEventListener("visibilitychange", function () { if (!document.hidden) tick(); });
    });
  }

  function init() {
    setupLogos();
    setupIntro();
    var hero = $("[data-hero]");
    if (hero) {
      // The intro wipes upward for ~1s; start the hero as it clears.
      whenIntroDone(function () {
        if (introPlaying === false) hero.style.setProperty("--hero-delay", "250ms");
        requestAnimationFrame(function () { hero.classList.add("is-ready"); });
      });
    }
    setupHeader();
    setupReveal();
    setupCounters();
    setupPointerFx();
    setupFocus();
    setupProfiles();
    setupForms();
    setupLive();
    $$("canvas[data-globe]").forEach(function (c) { new Globe(c); });
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init);
  else init();
})();
