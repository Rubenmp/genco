package org.test;

import java.io.File;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.time.LocalDate;
import java.time.LocalTime;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) {
        // A logger should be used here
        testInputOutput();

        testBasicVariables();
        final List<Integer> integerList = testCompoundVariables();
        testFlowControl(integerList);
        testSpecificOperators();
        testReservedKeywords();

        callThread();
    }


    private static void testInputOutput() {
        System.out.println("Hello world!");
        File myObj = new File("filename.txt");
    }


    /**
     * Basic variables testing
     */
    private static void testBasicVariables() {
        int intVarName;
        int intVarNameInit = 0 * 10 / 2;
        final int intVarNameFinal = -10;
        int intVarName1, intVarName2 = (intVarNameInit + 1 % 1);
        Integer integerVarName;

        float floatVarName;
        float floatVarNameInit = 0;
        final float floatVarNameFinal = -10;
        float floatVarName1, floatVarName2 = 0;
        Float FloatVarName;

        double doubleVarName;
        double doubleVarNameInit = 1.0;
        double doubleVarName1 = -1, doubleVarName2;
        Double DoubleVarName;

        long longVarName;
        long longVarNameInit = 10L;
        long longVarName1, longVarName2 = 5L, longVarNameCasting = Long.valueOf(1L);
        Long LongVarName;

        char charVarName;
        char charVarNameInit = 'c';
        char charVarName1, charVarName2 = 'd';
        Character characterVarName;

        boolean boolVarName;
        boolean boolVarNameInit = true;
        boolean boolVarName1, boolVarName2 = true, boolVarName3 = false, boolVarNameCasting = Boolean.TRUE;
        boolean boolVarCompound = false;
        boolVarCompound |= boolVarNameInit;
        boolVarCompound &= boolVarNameInit;
        Boolean BooleanVarName;

        String stringVarName;
        String stringVarInit = "something";
        String stringVarName1, stringVarName2 = null, stringVarName3 = (1 < 2) ? "Good day." : "Good evening.";
        ;

        testNonBasicVariables();
    }

    private static void testNonBasicVariables() {
        byte[] byteVarName;
        short shortVarName;
        LocalDate localDateVarName = LocalDate.now(); // Create a date object
        LocalTime localTimeVarName = LocalTime.now();
    }

    /* This method tests compound variables */
    public static List<Integer> testCompoundVariables(String... args) {
        final Integer integerVarName1 = null, integerVarName2 = 1;
        final List<Integer> integerList = List.of(integerVarName2); // Do not remove inline comment
        integerList.add(4);

        int[][] multidimensionalArrayOfNumbers = {{1, 2, 3, 4}, {5, 6, 7}};
        HashMap<String, String> capitalCities = new HashMap<String, String>();

        final Class<TestBaseClass> classVar = TestBaseClass.class;

        return integerList;
    }


    private static int testFlowControl(final List<Integer> integerList) throws RuntimeException {
        if (integerList == null || integerList.isEmpty()) {
            return 0;
        }

        if (true != false && !!(1 >= 1) || (null instanceof Integer)) {
            try {
                return (int) integerList.stream()
                        .map(t -> t).map(TestClass::identity).filter(t -> t > 0)
                        .collect(Collectors.toList()).stream().distinct().count();
            } catch (Exception e) {
                throw new RuntimeException(e);
            } finally {
                // Ignore
            }
        } else if ((1 + 2 - 1) == 2 && "string".equals("string") || 1 <= 0) {
            var counter = testLoops();
            return counter;
        } else {
            int day = 4;
            switch (day) {
                case 6:
                    System.out.println("Today is Saturday");
                    break;
                default:
                    System.out.println("Looking forward to the Weekend");
            }
            switch (day) {
                case 7 -> System.out.println("Today is Sunday");
                default -> System.out.println("Looking forward to the Weekend");
            }
        }

        return 0;
    }

    private static void testSpecificOperators() {
        System.out.println(10 >> 2);//10/2^2=10/4=2
        System.out.println(10 << 2);
        System.out.println(20 >>> 2);
        int a = 10, b = 5, c = 20;
        System.out.println(a < b & a < c);//false & true = false
        System.out.println(a < b && a++ < c);//false && true = false
        System.out.println(a > b || a < c);//true || true = true
        System.out.println(~b);// [b=10] 9 (positive of total minus, positive starts from 0)
    }

    private static void testReservedKeywords() {
        // Missing keywords: exports, goto, module, native, requires, strictfp

        assert (1 == 1);
    }


    private static void callThread() {
        Main obj = new Main();
        Thread thread = new Thread((Runnable) obj);
        thread.start();
    }

    private synchronized static int testLoops() {
        int counter = 0;
        for (int i = 0; i < 10; i++) {
            counter += i;
            continue;
        }
        for (int j = 10; j >= 10; --j) {
            counter -= j;
            counter *= 1;
            counter /= 1;
            counter %= 1;
            counter ^= 1;
            counter <<= 1;
            counter >>= 1;
            counter >>>= 1;
            break;
        }

        String[] cars = {"Volvo", "BMW", "Ford", "Mazda"};
        for (String i : cars) {
            System.out.println(i);
        }

        int i = 0;
        do {
            System.out.println(i);
            i++;
            counter = counter + 1;
        } while (i < 5);

        int j = 0;
        while (j < 5) {
            System.out.println(i);
            j++;
            counter = counter + 1;
        }

        return counter;
    }

    class TestClass extends TestBaseClass implements TestInterface {
        volatile int testClassIntVarName = 0;
        private transient TestEnum enumVarName = TestEnum.SOMETHING;
        Class<? extends TestInterface> yourCustomType;

        private TestClass() {
            super();
        }

        @Override
        public int identityMethodInterface(final int number) {
            return number + this.testClassIntVarName;
        }

        public static <T> T identity(T generic) {
            return generic;
        }
    }

    abstract class TestBaseClass {
    }


    interface TestInterface {
        default int identityMethodInterface(final int number) {
            return number;
        }
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.METHOD)
    public @interface Init {
    }

    enum TestEnum {
        SOMETHING, SIMILAR
    }
}
