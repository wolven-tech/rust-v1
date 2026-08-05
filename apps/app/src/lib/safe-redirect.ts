/**
 * Helpers for building redirect targets that cannot be pointed at an
 * attacker-controlled origin.
 *
 * Both the `next` query parameter and the `x-forwarded-host` header are
 * attacker-controlled on any request, so neither may be interpolated into a
 * redirect URL without validation.
 */

const DEFAULT_PATH = "/dashboard";

// Any origin works here as long as it can never be a real redirect target; it
// exists only so the URL parser can tell us whether `next` escaped it.
const PROBE_ORIGIN = "https://redirect-probe.invalid";

/**
 * Normalize a user-supplied `next` value to a same-origin path.
 *
 * Anything that resolves to a different origin — absolute URLs, protocol
 * relative URLs (`//evil.com`), backslash variants that browsers treat as
 * slashes (`/\evil.com`), or `javascript:` style schemes — falls back to
 * `fallback`.
 */
export function safeRedirectPath(
  next: string | null | undefined,
  fallback: string = DEFAULT_PATH,
): string {
  if (!next || !next.startsWith("/")) {
    return fallback;
  }

  let parsed: URL;
  try {
    parsed = new URL(next, PROBE_ORIGIN);
  } catch {
    return fallback;
  }

  // The parser resolves protocol-relative and absolute inputs away from the
  // probe origin, which is exactly the case we must reject.
  if (parsed.origin !== PROBE_ORIGIN) {
    return fallback;
  }

  // `/..//evil.com` resolves to the pathname `//evil.com`, which is
  // protocol-relative again the moment it is re-parsed against an origin.
  if (parsed.pathname.startsWith("//")) {
    return fallback;
  }

  return `${parsed.pathname}${parsed.search}${parsed.hash}`;
}

/**
 * Hosts that may be used as the redirect origin, in addition to the origin of
 * the incoming request.
 *
 * `VERCEL_URL` covers preview deployments where the request reaches the app
 * through a proxy; `ALLOWED_REDIRECT_HOSTS` is a comma-separated escape hatch
 * for custom domains.
 */
function allowedHosts(): Set<string> {
  const hosts = new Set<string>();

  for (const value of [
    process.env.VERCEL_URL,
    process.env.VERCEL_PROJECT_PRODUCTION_URL,
    ...(process.env.ALLOWED_REDIRECT_HOSTS?.split(",") ?? []),
  ]) {
    const host = value
      ?.trim()
      .replace(/^https?:\/\//, "")
      .replace(/\/.*$/, "")
      .toLowerCase();

    if (host) {
      hosts.add(host);
    }
  }

  return hosts;
}

/**
 * Resolve the origin to redirect to.
 *
 * `x-forwarded-host` is only honoured when it matches the request's own host or
 * appears in the allowlist; otherwise the request's origin is used.
 */
export function safeRedirectOrigin(
  origin: string,
  forwardedHost: string | null | undefined,
): string {
  if (!forwardedHost) {
    return origin;
  }

  // A proxy chain can append hosts; only the first one is meaningful.
  const host = forwardedHost.split(",")[0]?.trim().toLowerCase();
  if (!host) {
    return origin;
  }

  let requestHost: string;
  try {
    requestHost = new URL(origin).host.toLowerCase();
  } catch {
    return origin;
  }

  if (host === requestHost) {
    return origin;
  }

  return allowedHosts().has(host) ? `https://${host}` : origin;
}
