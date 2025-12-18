"use client";

import { Button } from "@rust-v1/ui/button";
import { Icons } from "@rust-v1/ui/icons";
import { Input } from "@rust-v1/ui/input";
import { useState } from "react";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4400";

interface Product {
  id: string;
  name: string;
  price?: number;
  description?: string;
}

interface SearchResult {
  query: string;
  results: Product[];
}

export function ProductSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<Product[]>([]);
  const [loading, setLoading] = useState(false);
  const [searched, setSearched] = useState(false);

  const handleSearch = async () => {
    if (!query.trim()) return;

    setLoading(true);
    setSearched(true);

    try {
      const response = await fetch(`${API_URL}/api/products/search`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ query }),
      });

      const data: SearchResult = await response.json();
      setResults(data.results || []);
    } catch (error) {
      console.error("Search failed:", error);
      setResults([]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3 mb-2">
        <div className="p-2 rounded-lg bg-purple-500/20">
          <Icons.Search className="size-5 text-purple-400" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-white">Product Search</h2>
          <p className="text-sm text-slate-400">Find products in the catalog</p>
        </div>
      </div>

      <div className="flex gap-2">
        <Input
          placeholder="Search products..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSearch()}
          className="flex-1 bg-white/5 border-white/10 text-white placeholder:text-slate-500"
        />
        <Button onClick={handleSearch} disabled={loading}>
          {loading ? (
            <span data-testid="search-loading">
              <Icons.Loader className="size-4 animate-spin" />
            </span>
          ) : (
            "Search"
          )}
        </Button>
      </div>

      {searched && (
        <div data-testid="search-results" className="space-y-2">
          {results.length === 0 ? (
            <p className="text-slate-400 text-sm py-4 text-center">
              No products found
            </p>
          ) : (
            <div className="grid gap-2">
              {results.map((product) => (
                <div
                  key={product.id}
                  data-testid="product-item"
                  className="p-4 rounded-xl bg-white/5 border border-white/10 hover:border-purple-500/30 transition-colors"
                >
                  <div className="flex items-center justify-between">
                    <p className="font-medium text-white">{product.name}</p>
                    <span className="text-xs text-slate-500 font-mono">
                      #{product.id}
                    </span>
                  </div>
                  {product.description && (
                    <p className="text-sm text-slate-400 mt-1">
                      {product.description}
                    </p>
                  )}
                  {product.price && (
                    <p className="text-sm font-semibold text-purple-400 mt-2">
                      ${product.price}
                    </p>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
