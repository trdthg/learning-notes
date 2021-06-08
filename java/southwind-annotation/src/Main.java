import java.lang.annotation.Annotation;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

public class Main {
    public static void main(String[] args) {
        try {
            Class<?> clazz = Class.forName("Foo");
            Annotation annotation = clazz.getAnnotation(InitAnno.class);
            if (annotation == null) {
                System.out.println("类前没有InitAnno注解");
            }
            Method[] methods = clazz.getMethods();
            for (Method method : methods) {
                boolean isInitAnno = method.isAnnotationPresent(InitAnno.class);
                if (isInitAnno) {
                    method.invoke(clazz.getConstructor(null).newInstance(null), null);
                }
            }
        } catch (ClassNotFoundException | InvocationTargetException | IllegalAccessException | InstantiationException | NoSuchMethodException e) {
            e.printStackTrace();
        }
    }
}
