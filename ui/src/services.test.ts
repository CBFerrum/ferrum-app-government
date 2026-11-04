import { describe, expect, it } from "vitest";
import { byUrgency } from "./Home";
import { stillNeeded } from "./Portal";

describe("queues and services", () => {
  it("puts overdue cases first, then the soonest due", () => {
    const l = byUrgency([
      { id: "late", overdue: false, due_on: "2026-11-20" },
      { id: "over", overdue: true, due_on: "2026-11-01" },
      { id: "soon", overdue: false, due_on: "2026-11-10" },
    ]);
    expect(l.map((c) => c.id)).toEqual(["over", "soon", "late"]);
  });

  it("lists the documents a draft still needs", () => {
    const app = { id: "a", documents: [{ document: "Site plan", file: "/files/1" }, { document: "ID", file: null }] };
    expect(stillNeeded(["Site plan", "ID", "Fire report"], app)).toEqual(["ID", "Fire report"]);
  });
});
