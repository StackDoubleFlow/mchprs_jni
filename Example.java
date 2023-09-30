public class Example {
    public static void main(String[] args) {
        Redpiler redpiler = new Redpiler();
        Redpiler.TickEntry[] ticks = new Redpiler.TickEntry[] {
            new Redpiler.TickEntry(0, 1, 1, 0, 1)
        };
        redpiler.initializeWorld(2, 1, 2, new int[] {
            2114, 16014, // Wire, Target
            2114, 3960, // Wire, Wall Torch (South Lit)
        }, ticks);
        redpiler.compileWorld(false, false);
        redpiler.runTicks(2);
        redpiler.flush(new Redpiler.BlockChangeConsumer() {
            @Override
            public void onBlockChange(int xPos, int yPos, int zPos, int newState) {
                System.out.println("Block at (" + xPos + "," + yPos + "," + zPos + ") changed to state " + newState);
            }
        });
    }
}
