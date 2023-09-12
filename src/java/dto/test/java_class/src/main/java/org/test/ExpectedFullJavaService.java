package org.test;

import org.test.JavaClassFrom;
import org.springframework.stereotype.Service;
import org.test.JavaInterfaceForClass;

@Service
public class FullJavaService extends JavaClassFrom implements JavaInterfaceForClass {
    private boolean field;

    int newMethod() {
    }
}
