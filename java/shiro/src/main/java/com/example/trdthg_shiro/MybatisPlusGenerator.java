package com.example.trdthg_shiro;


import com.baomidou.mybatisplus.annotation.DbType;
import com.baomidou.mybatisplus.generator.AutoGenerator;
import com.baomidou.mybatisplus.generator.config.DataSourceConfig;
import com.baomidou.mybatisplus.generator.config.GlobalConfig;
import com.baomidou.mybatisplus.generator.config.PackageConfig;
import com.baomidou.mybatisplus.generator.config.StrategyConfig;
import com.baomidou.mybatisplus.generator.config.rules.NamingStrategy;

public class MybatisPlusGenerator {
    public static void main(String[] args) {
        AutoGenerator autoGenerator = new AutoGenerator();
        // 数据源
        autoGenerator.setDataSource(new DataSourceConfig()
                .setDbType(DbType.MYSQL)
                .setUrl("jdbc:mysql://localhost:3306/library1?serverTimezone=GMT")
                .setUsername("root")
                .setPassword("hj689753")
                .setDriverName("com.mysql.cj.jdbc.Driver")
        );
        // 全局配置
        autoGenerator.setGlobalConfig(new GlobalConfig()
                .setOutputDir( "src/main/java")
                .setOpen(false)
                .setAuthor("trdthg")
                .setServiceName("%sService")  // 设定service类名不带I
        );
        // 包信息
        autoGenerator.setPackageInfo(new PackageConfig()
                .setParent("com.example.trdthg_shiro")
                .setModuleName("")
                .setController("controller")
                .setEntity("entity")
                .setMapper("mapper")
                .setService("service")
                .setServiceImpl("service.impl")
        );
        // 策略配置
        autoGenerator.setStrategy(new StrategyConfig()
                .setEntityLombokModel(true)
                .setColumnNaming(NamingStrategy.underline_to_camel) //数据库下划线自动转entity驼峰
                .setNaming(NamingStrategy.underline_to_camel)
        );
        autoGenerator.execute();
    }
}
