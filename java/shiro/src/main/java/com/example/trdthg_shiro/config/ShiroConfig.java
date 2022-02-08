package com.example.trdthg_shiro.config;

import com.example.trdthg_shiro.filter.MyAuthenticatingFilter;
import com.example.trdthg_shiro.realm.ShiroUserRealm;
import org.apache.shiro.spring.web.ShiroFilterFactoryBean;
import org.apache.shiro.web.mgt.DefaultWebSecurityManager;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import javax.servlet.Filter;
import java.util.HashMap;
import java.util.Hashtable;
import java.util.LinkedHashMap;
import java.util.Map;

@Configuration
public class ShiroConfig {

    @Bean
    public ShiroFilterFactoryBean shiroFilterFactoryBean(@Qualifier("defaultWebSecurityManager") DefaultWebSecurityManager defaultWebSecurityManager) {
        ShiroFilterFactoryBean shiroFilterFactoryBean = new ShiroFilterFactoryBean();
        shiroFilterFactoryBean.setSecurityManager(defaultWebSecurityManager);
        // 自定义authc过滤器
        Map<String, Filter> filters = new HashMap<>();
        filters.put("authc", new MyAuthenticatingFilter());
        shiroFilterFactoryBean.setFilters(filters);
        // 自定义role过滤器

        // 权限设置
        Map<String, String> map = new Hashtable<>();
        map.put("/select", "authc,perms[select]");
        map.put("/vip", "authc,roles[vip]");
        map.put("/login", "anon");
        shiroFilterFactoryBean.setFilterChainDefinitionMap(map);
        shiroFilterFactoryBean.setLoginUrl("/login");
        return shiroFilterFactoryBean;
    }

    @Bean
    public DefaultWebSecurityManager defaultWebSecurityManager(@Qualifier("shiroUserRealm") ShiroUserRealm shiroUserRealm) {
        DefaultWebSecurityManager defaultWebSecurityManager = new DefaultWebSecurityManager();
        defaultWebSecurityManager.setRealm(shiroUserRealm);
        return defaultWebSecurityManager;
    }

    @Bean
    public ShiroUserRealm shiroUserRealm() {
        return new ShiroUserRealm();
    }

}
