package com.example.trdthg_shiro.realm;

import com.example.trdthg_shiro.entity.ShiroPermission;
import com.example.trdthg_shiro.entity.ShiroRole;
import com.example.trdthg_shiro.entity.ShiroUser;
import com.example.trdthg_shiro.service.ShiroPermissionService;
import com.example.trdthg_shiro.service.ShiroRoleService;
import com.example.trdthg_shiro.service.ShiroUserService;
import org.apache.shiro.SecurityUtils;
import org.apache.shiro.authc.*;
import org.apache.shiro.authz.AuthorizationInfo;
import org.apache.shiro.authz.SimpleAuthorizationInfo;
import org.apache.shiro.realm.AuthorizingRealm;
import org.apache.shiro.subject.PrincipalCollection;
import org.apache.shiro.subject.Subject;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.redis.core.RedisTemplate;

import java.util.List;

public class ShiroUserRealm extends AuthorizingRealm {

    @Autowired
    RedisTemplate redisTemplate;
    @Autowired
    ShiroUserService shiroUserService;
    @Autowired
    ShiroRoleService shiroRoleService;
    @Autowired
    ShiroPermissionService shiroPermissionService;

    @Override
    protected AuthorizationInfo doGetAuthorizationInfo(PrincipalCollection principals) {
        //1. 从 PrincipalCollection 中来获取登录用户的信息
        ShiroUser shiroUser = (ShiroUser) principals.getPrimaryPrincipal();
        System.out.println(shiroUser.getUsername());
        //2.添加角色和权限
        SimpleAuthorizationInfo simpleAuthorizationInfo = new SimpleAuthorizationInfo();
        List<String> roles = (List<String>) redisTemplate.opsForValue().get("roles_" + shiroUser);
        if (roles == null) {
            roles = shiroRoleService.findByUserId(shiroUser.getId());
            redisTemplate.opsForValue().set("roles_" + shiroUser, roles);
        }
        for (String role : roles) {
            //2.1添加角色
            simpleAuthorizationInfo.addRole(role);
        }
        List<String> permissions = (List<String>) redisTemplate.opsForValue().get("permissions_" + shiroUser);
        if (permissions == null) {
            permissions = shiroPermissionService.findByUserId(shiroUser.getId());
        }
        for (String permission : permissions) {
            simpleAuthorizationInfo.addStringPermission(permission);
        }
        System.out.println("权限校验完成");
        return simpleAuthorizationInfo;
    }

    @Override
    protected AuthenticationInfo doGetAuthenticationInfo(AuthenticationToken token) throws AuthenticationException {
        //获取token，既前端传入的token
        String access_token = (String) token.getPrincipal();
        String username = (String) shiroUserService.getInfoFromToken(access_token).get("username");
        //1. 根据username，查询用户token是否存在
        String old_access_token = (String) redisTemplate.opsForValue().get("access_token" + username);
        System.out.println(access_token);
        if (old_access_token != null && access_token.equals(old_access_token)) {
            ShiroUser user = shiroUserService.findByUsername(username);
            if (user == null) {
                throw new UnknownAccountException("用户不存在!");
            }
            return new SimpleAuthenticationInfo(user, access_token, this.getName());
        }
        System.out.println("登录校验完成");
        throw new IncorrectCredentialsException("token失效，请重新登录");
    }

}
