export function Footer() {
  return (
    <footer className="flex items-center justify-center font-mono text-xs fixed bottom-8 w-full flex-col space-y-4">
      <span className="text-[#878787]">Featuring</span>

      <div className="flex items-center space-x-6">
        <a
          href="https://www.rust-lang.org"
          target="_blank"
          rel="noopener noreferrer"
          className="text-[#F5F5F3] hover:underline"
        >
          Rust
        </a>
        <a
          href="https://github.com/all-source-os"
          target="_blank"
          rel="noopener noreferrer"
          className="text-[#F5F5F3] hover:underline"
        >
          AllSource
        </a>
        <a
          href="https://all-source-os.github.io/all-frame/"
          target="_blank"
          rel="noopener noreferrer"
          className="text-[#F5F5F3] hover:underline"
        >
          Allframe
        </a>
      </div>
    </footer>
  );
}
