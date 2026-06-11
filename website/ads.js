/*
  AdSense bootstrap for all pages.
  Replace these values before going live.
*/
const ADSENSE_CLIENT = "ca-pub-REPLACE_ME";
const ADS_ENABLED = ADSENSE_CLIENT.startsWith("ca-pub-") && ADSENSE_CLIENT !== "ca-pub-REPLACE_ME";

function loadAdSenseScript() {
  if (!ADS_ENABLED) return;
  if (document.querySelector("script[data-adsense-loader='1']")) return;
  const s = document.createElement("script");
  s.async = true;
  s.dataset.adsenseLoader = "1";
  s.src =
    "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=" +
    encodeURIComponent(ADSENSE_CLIENT);
  s.crossOrigin = "anonymous";
  document.head.appendChild(s);
}

function mountAds() {
  const slots = Array.from(document.querySelectorAll(".ad-slot[data-ad-slot]"));
  if (!slots.length) return;

  for (const box of slots) {
    if (!ADS_ENABLED) continue;
    const adSlot = box.getAttribute("data-ad-slot");
    if (!adSlot) continue;

    box.innerHTML = "";
    const ins = document.createElement("ins");
    ins.className = "adsbygoogle";
    ins.style.display = "block";
    ins.dataset.adClient = ADSENSE_CLIENT;
    ins.dataset.adSlot = adSlot;
    ins.dataset.adFormat = "auto";
    ins.dataset.fullWidthResponsive = "true";
    box.appendChild(ins);
  }

  if (!ADS_ENABLED) return;
  for (let i = 0; i < slots.length; i += 1) {
    try {
      (window.adsbygoogle = window.adsbygoogle || []).push({});
    } catch (_) {
      // Ignore during local preview before AdSense script initialization.
    }
  }
}

document.addEventListener("DOMContentLoaded", () => {
  loadAdSenseScript();
  mountAds();
});
