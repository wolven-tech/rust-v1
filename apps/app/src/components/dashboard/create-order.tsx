"use client";

import type { Order, Product } from "@/domain/api";
import { useCreateOrder, useSearchProducts } from "@/hooks/api";
import { Button } from "@rust-v1/ui/button";
import { Icons } from "@rust-v1/ui/icons";
import { Input } from "@rust-v1/ui/input";
import { useState } from "react";

export function CreateOrder() {
  const [query, setQuery] = useState("");
  const [products, setProducts] = useState<Product[]>([]);
  const [selectedProduct, setSelectedProduct] = useState<Product | null>(null);
  const [quantity, setQuantity] = useState("1");
  const [searched, setSearched] = useState(false);
  const [orderResult, setOrderResult] = useState<Order | null>(null);

  const searchMutation = useSearchProducts();
  const orderMutation = useCreateOrder();

  const handleSearch = async () => {
    if (!query.trim()) return;

    setSearched(true);
    setSelectedProduct(null);

    searchMutation.mutate(
      { query },
      {
        onSuccess: (data: { results: Product[] }) => {
          setProducts(data.results || []);
        },
        onError: () => {
          setProducts([]);
        },
      },
    );
  };

  const handleCreateOrder = async () => {
    if (!selectedProduct) return;

    const quantityNum = Number.parseInt(quantity, 10);
    if (Number.isNaN(quantityNum) || quantityNum <= 0) return;

    orderMutation.mutate(
      { product: selectedProduct.name, quantity: quantityNum },
      {
        onSuccess: (data: Order) => {
          setOrderResult(data);
          setSelectedProduct(null);
          setQuery("");
          setProducts([]);
          setSearched(false);
          setQuantity("1");
        },
      },
    );
  };

  const error = searchMutation.error?.message || orderMutation.error?.message;
  const isSearching = searchMutation.isPending;
  const isOrdering = orderMutation.isPending;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <div className="p-2 rounded-lg bg-gradient-to-br from-purple-500/20 to-blue-500/20">
          <Icons.Plus className="size-5 text-purple-400" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-white">Create Order</h2>
          <p className="text-sm text-slate-400">
            Search products and place an order
          </p>
        </div>
      </div>

      {/* Search Section */}
      <div className="space-y-4">
        <div className="flex gap-2">
          <Input
            placeholder="Search products (e.g. widget, gadget, sensor)..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            className="flex-1 bg-white/5 border-white/10 text-white placeholder:text-slate-500"
          />
          <Button onClick={handleSearch} disabled={isSearching}>
            {isSearching ? (
              <Icons.Loader className="size-4 animate-spin" />
            ) : (
              <>
                <Icons.Search className="size-4 mr-2" />
                Search
              </>
            )}
          </Button>
        </div>

        {/* Product Results */}
        {searched && (
          <div className="space-y-2">
            {products.length === 0 ? (
              <p className="text-slate-400 text-sm py-4 text-center">
                No products found. Try "widget", "gadget", or "sensor".
              </p>
            ) : (
              <div className="grid gap-2 max-h-48 overflow-y-auto">
                {products.map((product) => (
                  <button
                    key={product.id}
                    type="button"
                    onClick={() => setSelectedProduct(product)}
                    className={`p-3 rounded-lg text-left transition-all ${
                      selectedProduct?.id === product.id
                        ? "bg-purple-500/20 border-purple-500/50 border"
                        : "bg-white/5 border border-white/10 hover:border-purple-500/30"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium text-white">
                        {product.name}
                      </span>
                      <span className="text-xs text-slate-500 font-mono">
                        #{product.id}
                      </span>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Order Section */}
      {selectedProduct && (
        <div className="p-4 rounded-xl bg-white/5 border border-white/10 space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-slate-500 uppercase">
                Selected Product
              </p>
              <p className="text-white font-medium">{selectedProduct.name}</p>
            </div>
            <button
              type="button"
              onClick={() => setSelectedProduct(null)}
              className="text-slate-400 hover:text-white text-sm"
            >
              Change
            </button>
          </div>

          <div className="flex gap-3">
            <div className="flex-1">
              <label className="text-xs text-slate-500 uppercase block mb-1">
                Quantity
              </label>
              <Input
                type="number"
                min="1"
                value={quantity}
                onChange={(e) => setQuantity(e.target.value)}
                className="bg-white/5 border-white/10 text-white"
              />
            </div>
            <div className="flex items-end">
              <Button
                onClick={handleCreateOrder}
                disabled={isOrdering}
                className="gap-2"
              >
                {isOrdering ? (
                  <Icons.Loader className="size-4 animate-spin" />
                ) : (
                  <>
                    <Icons.Check className="size-4" />
                    Place Order
                  </>
                )}
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Error Message */}
      {error && (
        <div className="p-3 bg-red-500/10 text-red-400 rounded-xl border border-red-500/20">
          {error}
        </div>
      )}

      {/* Success Message */}
      {orderResult && (
        <div
          data-testid="order-success"
          className="p-4 rounded-xl bg-green-500/10 border border-green-500/20 space-y-3"
        >
          <div className="flex items-center gap-2">
            <div className="size-8 rounded-full bg-green-500/20 flex items-center justify-center">
              <Icons.Check className="size-4 text-green-400" />
            </div>
            <p className="font-medium text-green-400">Order Created!</p>
          </div>
          <div className="grid grid-cols-3 gap-3 text-sm">
            <div>
              <p className="text-slate-500 text-xs uppercase">Order ID</p>
              <p
                data-testid="order-id"
                className="font-mono text-white text-xs"
              >
                {orderResult.order_id.slice(0, 8)}...
              </p>
            </div>
            <div>
              <p className="text-slate-500 text-xs uppercase">Product</p>
              <p className="text-white">{orderResult.product}</p>
            </div>
            <div>
              <p className="text-slate-500 text-xs uppercase">Status</p>
              <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs bg-green-500/20 text-green-400 capitalize">
                {orderResult.status}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
