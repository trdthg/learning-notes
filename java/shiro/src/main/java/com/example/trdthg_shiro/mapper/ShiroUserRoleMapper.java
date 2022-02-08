package com.example.trdthg_shiro.mapper;

import com.example.trdthg_shiro.entity.ShiroRole;
import com.example.trdthg_shiro.entity.ShiroUser;
import com.example.trdthg_shiro.entity.ShiroUserRole;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import org.apache.ibatis.annotations.Select;
import org.springframework.stereotype.Repository;

import java.util.List;

/**
 * <p>
 *  Mapper 接口
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
@Repository
public interface ShiroUserRoleMapper extends BaseMapper<ShiroUserRole> {
//    @Select("Select p.*, u.id, u.name UserName from mybatis_plus_product p, mybatis_plus_user u where p.id=#{id}")
//    List<ShiroRole> findRolesByUser(ShiroUser shiroUser);
}
