export function Footer() {
  return (
    <div className="flex items-center justify-center font-mono text-xs fixed bottom-0 right-4 bottom-4 space-x-2">
      <p className="text-sm text-slate-500">
        Built by{" "}
        <a
          href="https://decebaldobrica.com"
          target="_blank"
          rel="noopener noreferrer"
          className="text-slate-400 hover:text-white transition-colors underline underline-offset-2"
        >
          Decebal Dobrica
        </a>
      </p>
    </div>
  );
}
