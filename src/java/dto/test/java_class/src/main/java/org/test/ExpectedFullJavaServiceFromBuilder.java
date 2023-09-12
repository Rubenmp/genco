package org.test;

import org.springframework.stereotype.Service;
import org.test.JavaClassFrom;
import org.test.JavaInterfaceForClass;

@Service
public class FullJavaServiceFromBuilder extends JavaClassFrom implements JavaInterfaceForClass {
    private boolean field;

    int newMethod() {
    }
}
