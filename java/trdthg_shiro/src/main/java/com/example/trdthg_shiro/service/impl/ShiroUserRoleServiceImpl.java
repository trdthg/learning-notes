package com.example.trdthg_shiro.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.example.trdthg_shiro.entity.ShiroUserRole;
import com.example.trdthg_shiro.mapper.ShiroUserRoleMapper;
import com.example.trdthg_shiro.service.ShiroUserRoleService;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import javax.xml.namespace.QName;
import java.util.List;

/**
 * <p>
 * 服务实现类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
@Service
public class ShiroUserRoleServiceImpl extends ServiceImpl<ShiroUserRoleMapper, ShiroUserRole> implements ShiroUserRoleService {

    @Autowired
    ShiroUserRoleMapper shiroUserRoleMapper;

    @Override
    public List<Integer> findByUserId(int id) {
        QueryWrapper queryWrapper = new QueryWrapper();
        queryWrapper.eq("user_id", id);
        return shiroUserRoleMapper.selectList(queryWrapper);
    }
}
