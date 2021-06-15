import java.security.SecureRandom;
import java.util.UUID;
public class Test {
    public static void main(String[] args) {
        SecureRandom random = new SecureRandom();
        byte seed[] = random.generateSeed(20);
        for (int i = 0; i < seed.length; i++) {
            System.out.print(seed[i]);
        }
        byte bytes[] = new byte[20];
        random.nextBytes(bytes);
        System.out.println(bytes.toString() + "  " + bytes.length);
        for (int i = 0; i < bytes.length; i++) {
            System.out.print(bytes[i]);
        }
        System.out.println("");
        UUID uuid = UUID.randomUUID();
        System.out.println(uuid.toString());
    }
}