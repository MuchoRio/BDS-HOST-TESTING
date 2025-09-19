<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>02. Page-Dashboard</name>
   <tag></tag>
   <elementGuidId>1b7d3f8e-0440-429f-901d-9821cca66831</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.fast-home.card.card-body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Sumber Penghasilan'])[1]/following::div[3]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>.fast-home</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>46d14e74-e8c3-46eb-a35b-308a5fdff529</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fast-home card card-body</value>
      <webElementGuid>18d7dca7-e97e-4621-87ed-c5f9dd74eabd</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
          

  
    
      
        
          
            
              18
            
            
          
          Nomor Referensi Transaksi
          
            
              
                
                
                  
                
              
            
          
        
      
    
  

  
    
    
  

  
    
      
        
          
            
              
              Tunai
            
          
        
      
      
        
          
            
              
              Overbooking
            
          
        
      
      
        
          
            
              
              Transfer
            
          
        
      
      
        
          
            
              
              Remittance
            
          
        
      
    
    
      
        
          
            
              
              Cek/Bilyet Giro
            
          
        
      
      
        
          
            
              
              Pembayaran
            
          
        
      
      
        
          
            
              
              Loan
            
          
        
      
      
        
          
            
              
              Lainnya
            
          
        
      
    
  

  
    
      
      
      
      
    

    
      
      
      
      
    
  
  
    
    
  


$(&quot;#fast-home a&quot;).on(&quot;click&quot;, function () {
  // _tranHttp($(this))
  if ($(this).attr(&quot;data-link&quot;) == &quot;2229002&quot;) _tranHttp($(this), null, true);
  else _tranHttp($(this));
});

$(&quot;#searching&quot;).on(&quot;input&quot;, function () {
  if ($(this).val().length > 18) {
    window.alert(&quot;Input transaksi tidak boleh lebih dari 18 karakter&quot;);
    $(this).val(&quot;&quot;);
  }
});

$(&quot;#btnSearch&quot;).click(function () {
  var nilai = $(&quot;#searching&quot;).val();
  if (nilai != &quot;&quot;) {
    if (nilai.length > 5) {
      brn = $(&quot;#brn-cod&quot;).text();
      today = new Date();
      yymmdd = [today.getFullYear().toString().substr(2), String(today.getMonth() + 1).padStart(2, &quot;0&quot;), String(today.getDate()).padStart(2, &quot;0&quot;)].join(&quot;&quot;);

      /* if (nilai.substr(8, 6) !== yymmdd) {
          window.alert(&quot;Nomor Referensi sudah tidak berlaku&quot;);
        } else */ if (nilai.substr(5, 3) == String(brn).padStart(3, &quot;0&quot;)) {
        var no_ref = nilai.toUpperCase();
        var tran = no_ref.substr(0, 5);
        checkRefTran(tran, no_ref);
      } else {
        window.alert(&quot;Kode transaksi tidak ditemukan&quot;);
      }
    } else {
      _tranHttp($('&lt;a href=&quot;#fast-tran&quot; data-link=&quot;' + nilai + '&quot;>&lt;/a>'));
    }
  } else {
    window.alert(&quot;Mohon Isi Transaksi&quot;);
  }
});

function checkRefTran(tran, ref) {
  /**
   * Upd:
   * - Natasha [22-Feb-2024]: BRD Enhancement Otorisasi Kuasa |Default jenis pelaku dihapus
   */
  var cod = 0;
  var jsondata = {};
  if (tran == &quot;TROVB&quot; || tran == &quot;TRRTG&quot; || tran == &quot;TRRMT&quot; || tran == &quot;TRSKN&quot;) {
    var datax = {
      tran: &quot;transfer&quot;,
      apa: tran.substr(0, 2).toLowerCase() + tran.charAt(2) + tran.slice(3).toLowerCase(),
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/transfer/get-transfer&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        if (tran == &quot;TROVB&quot;) {
          cod = 6; //73
        } else if (tran == &quot;TRRTG&quot;) {
          cod = res.sum_dana == 2 ? 12 : 8;
        } else if (tran == &quot;TRSKN&quot;) {
          cod = res.sum_dana == 2 ? 2 : 3;
        } else if (tran == &quot;TRRMT&quot;) {
          cod = 16;
        }

        jsondata = {
          ...res,
          jenisnasabah1: res.jenisnasabah1 == 1 ? &quot;I&quot; : &quot;P&quot;,
          no_id_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.idno1,
          nama_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname1,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.idtype1,
          // Natasha [22-Feb-2024]
          // jenis_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          isWic: res.hasOwnProperty(&quot;wic&quot;) ? res.wic.wic_cust : &quot;N&quot;,
          isWkl: res.kua.kua_cust,
        };

        fast.tranNext(cod, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;STTUN&quot;) {
    var datax = {
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/setor-tunai/get-setor-tunai&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        jsondata = {
          ...res,
          prodname2: res.accountType,
          idtype2: res.typeid,
          no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.idno2,
          // no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.kua.kua_cust == &quot;Y&quot; ? res.kua_id_no : res.idno2,
          nama_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname2,
          // Natasha [22-Feb-2024]
          // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
          isWic: res.wic.wic_cust,
          isWkl: res.kua.kua_cust,
        };
        console.log(`sttun ===`, jsondata);
        fast.tranNext(1, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;TRTUN&quot;) {
    var datax = {
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/tarik-tunai/get-tarik-tunai&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        jsondata = {
          ...res,
          prodname1: res.accountType,
          idtype1: res.typeid,
          no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : `${res[&quot;idno1&quot;]}`.replace(/^\s+|\s+$/gm, &quot;&quot;),
          nama_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname1,
          // Natasha [22-Feb-2024]
          //  jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
          isWic: res.wic.wic_cust,
          isWkl: res.kua.kua_cust,
        };
        fast.tranNext(4, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;BYBNT&quot;) {
    fast.getURL(&quot;/fastchannel/bank-notes/get-bn/bybnt/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(58, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  } else if (tran == &quot;SLBNT&quot;) {
    fast.getURL(&quot;/fastchannel/bank-notes/get-bn/slbnt/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(57, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  } else if (tran == &quot;PAYCC&quot;) {
    fast.getURL(&quot;/fastchannel/pay-mbranch/findRef/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            noCC: res.noBill,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(8901, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  }
}








        </value>
      <webElementGuid>6f886100-52e8-41c7-ba5b-3a8e3c47fc91</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;desktop&quot;]/div[@class=&quot;row&quot;]/div[@class=&quot;col Lmr-3&quot;]/div[@class=&quot;tab-content&quot;]/div[@class=&quot;fast-home card card-body&quot;]</value>
      <webElementGuid>f59a0a71-4460-48e5-a7f2-ea1548537f56</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Sumber Penghasilan'])[1]/following::div[3]</value>
      <webElementGuid>8f3ede8f-4b65-400a-819c-ecc6e669440b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Hubungan hukum dengan pemilik dana'])[1]/following::div[3]</value>
      <webElementGuid>824e7f71-0d4e-4489-ab52-71d3b0c59c75</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[6]/div[2]/div/div</value>
      <webElementGuid>402d99d3-1a42-44e1-b36b-d2005417f7a1</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
          

  
    
      
        
          
            
              18
            
            
          
          Nomor Referensi Transaksi
          
            
              
                
                
                  
                
              
            
          
        
      
    
  

  
    
    
  

  
    
      
        
          
            
              
              Tunai
            
          
        
      
      
        
          
            
              
              Overbooking
            
          
        
      
      
        
          
            
              
              Transfer
            
          
        
      
      
        
          
            
              
              Remittance
            
          
        
      
    
    
      
        
          
            
              
              Cek/Bilyet Giro
            
          
        
      
      
        
          
            
              
              Pembayaran
            
          
        
      
      
        
          
            
              
              Loan
            
          
        
      
      
        
          
            
              
              Lainnya
            
          
        
      
    
  

  
    
      
      
      
      
    

    
      
      
      
      
    
  
  
    
    
  


$(&quot;#fast-home a&quot;).on(&quot;click&quot;, function () {
  // _tranHttp($(this))
  if ($(this).attr(&quot;data-link&quot;) == &quot;2229002&quot;) _tranHttp($(this), null, true);
  else _tranHttp($(this));
});

$(&quot;#searching&quot;).on(&quot;input&quot;, function () {
  if ($(this).val().length > 18) {
    window.alert(&quot;Input transaksi tidak boleh lebih dari 18 karakter&quot;);
    $(this).val(&quot;&quot;);
  }
});

$(&quot;#btnSearch&quot;).click(function () {
  var nilai = $(&quot;#searching&quot;).val();
  if (nilai != &quot;&quot;) {
    if (nilai.length > 5) {
      brn = $(&quot;#brn-cod&quot;).text();
      today = new Date();
      yymmdd = [today.getFullYear().toString().substr(2), String(today.getMonth() + 1).padStart(2, &quot;0&quot;), String(today.getDate()).padStart(2, &quot;0&quot;)].join(&quot;&quot;);

      /* if (nilai.substr(8, 6) !== yymmdd) {
          window.alert(&quot;Nomor Referensi sudah tidak berlaku&quot;);
        } else */ if (nilai.substr(5, 3) == String(brn).padStart(3, &quot;0&quot;)) {
        var no_ref = nilai.toUpperCase();
        var tran = no_ref.substr(0, 5);
        checkRefTran(tran, no_ref);
      } else {
        window.alert(&quot;Kode transaksi tidak ditemukan&quot;);
      }
    } else {
      _tranHttp($(&quot; , &quot;'&quot; , &quot;&lt;a href=&quot;#fast-tran&quot; data-link=&quot;&quot; , &quot;'&quot; , &quot; + nilai + &quot; , &quot;'&quot; , &quot;&quot;>&lt;/a>&quot; , &quot;'&quot; , &quot;));
    }
  } else {
    window.alert(&quot;Mohon Isi Transaksi&quot;);
  }
});

function checkRefTran(tran, ref) {
  /**
   * Upd:
   * - Natasha [22-Feb-2024]: BRD Enhancement Otorisasi Kuasa |Default jenis pelaku dihapus
   */
  var cod = 0;
  var jsondata = {};
  if (tran == &quot;TROVB&quot; || tran == &quot;TRRTG&quot; || tran == &quot;TRRMT&quot; || tran == &quot;TRSKN&quot;) {
    var datax = {
      tran: &quot;transfer&quot;,
      apa: tran.substr(0, 2).toLowerCase() + tran.charAt(2) + tran.slice(3).toLowerCase(),
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/transfer/get-transfer&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        if (tran == &quot;TROVB&quot;) {
          cod = 6; //73
        } else if (tran == &quot;TRRTG&quot;) {
          cod = res.sum_dana == 2 ? 12 : 8;
        } else if (tran == &quot;TRSKN&quot;) {
          cod = res.sum_dana == 2 ? 2 : 3;
        } else if (tran == &quot;TRRMT&quot;) {
          cod = 16;
        }

        jsondata = {
          ...res,
          jenisnasabah1: res.jenisnasabah1 == 1 ? &quot;I&quot; : &quot;P&quot;,
          no_id_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.idno1,
          nama_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname1,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.idtype1,
          // Natasha [22-Feb-2024]
          // jenis_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          isWic: res.hasOwnProperty(&quot;wic&quot;) ? res.wic.wic_cust : &quot;N&quot;,
          isWkl: res.kua.kua_cust,
        };

        fast.tranNext(cod, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;STTUN&quot;) {
    var datax = {
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/setor-tunai/get-setor-tunai&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        jsondata = {
          ...res,
          prodname2: res.accountType,
          idtype2: res.typeid,
          no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.idno2,
          // no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.kua.kua_cust == &quot;Y&quot; ? res.kua_id_no : res.idno2,
          nama_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname2,
          // Natasha [22-Feb-2024]
          // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
          isWic: res.wic.wic_cust,
          isWkl: res.kua.kua_cust,
        };
        console.log(`sttun ===`, jsondata);
        fast.tranNext(1, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;TRTUN&quot;) {
    var datax = {
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/tarik-tunai/get-tarik-tunai&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        jsondata = {
          ...res,
          prodname1: res.accountType,
          idtype1: res.typeid,
          no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : `${res[&quot;idno1&quot;]}`.replace(/^\s+|\s+$/gm, &quot;&quot;),
          nama_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname1,
          // Natasha [22-Feb-2024]
          //  jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
          isWic: res.wic.wic_cust,
          isWkl: res.kua.kua_cust,
        };
        fast.tranNext(4, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;BYBNT&quot;) {
    fast.getURL(&quot;/fastchannel/bank-notes/get-bn/bybnt/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(58, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  } else if (tran == &quot;SLBNT&quot;) {
    fast.getURL(&quot;/fastchannel/bank-notes/get-bn/slbnt/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(57, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  } else if (tran == &quot;PAYCC&quot;) {
    fast.getURL(&quot;/fastchannel/pay-mbranch/findRef/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            noCC: res.noBill,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(8901, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  }
}








        &quot;) or . = concat(&quot;
          

  
    
      
        
          
            
              18
            
            
          
          Nomor Referensi Transaksi
          
            
              
                
                
                  
                
              
            
          
        
      
    
  

  
    
    
  

  
    
      
        
          
            
              
              Tunai
            
          
        
      
      
        
          
            
              
              Overbooking
            
          
        
      
      
        
          
            
              
              Transfer
            
          
        
      
      
        
          
            
              
              Remittance
            
          
        
      
    
    
      
        
          
            
              
              Cek/Bilyet Giro
            
          
        
      
      
        
          
            
              
              Pembayaran
            
          
        
      
      
        
          
            
              
              Loan
            
          
        
      
      
        
          
            
              
              Lainnya
            
          
        
      
    
  

  
    
      
      
      
      
    

    
      
      
      
      
    
  
  
    
    
  


$(&quot;#fast-home a&quot;).on(&quot;click&quot;, function () {
  // _tranHttp($(this))
  if ($(this).attr(&quot;data-link&quot;) == &quot;2229002&quot;) _tranHttp($(this), null, true);
  else _tranHttp($(this));
});

$(&quot;#searching&quot;).on(&quot;input&quot;, function () {
  if ($(this).val().length > 18) {
    window.alert(&quot;Input transaksi tidak boleh lebih dari 18 karakter&quot;);
    $(this).val(&quot;&quot;);
  }
});

$(&quot;#btnSearch&quot;).click(function () {
  var nilai = $(&quot;#searching&quot;).val();
  if (nilai != &quot;&quot;) {
    if (nilai.length > 5) {
      brn = $(&quot;#brn-cod&quot;).text();
      today = new Date();
      yymmdd = [today.getFullYear().toString().substr(2), String(today.getMonth() + 1).padStart(2, &quot;0&quot;), String(today.getDate()).padStart(2, &quot;0&quot;)].join(&quot;&quot;);

      /* if (nilai.substr(8, 6) !== yymmdd) {
          window.alert(&quot;Nomor Referensi sudah tidak berlaku&quot;);
        } else */ if (nilai.substr(5, 3) == String(brn).padStart(3, &quot;0&quot;)) {
        var no_ref = nilai.toUpperCase();
        var tran = no_ref.substr(0, 5);
        checkRefTran(tran, no_ref);
      } else {
        window.alert(&quot;Kode transaksi tidak ditemukan&quot;);
      }
    } else {
      _tranHttp($(&quot; , &quot;'&quot; , &quot;&lt;a href=&quot;#fast-tran&quot; data-link=&quot;&quot; , &quot;'&quot; , &quot; + nilai + &quot; , &quot;'&quot; , &quot;&quot;>&lt;/a>&quot; , &quot;'&quot; , &quot;));
    }
  } else {
    window.alert(&quot;Mohon Isi Transaksi&quot;);
  }
});

function checkRefTran(tran, ref) {
  /**
   * Upd:
   * - Natasha [22-Feb-2024]: BRD Enhancement Otorisasi Kuasa |Default jenis pelaku dihapus
   */
  var cod = 0;
  var jsondata = {};
  if (tran == &quot;TROVB&quot; || tran == &quot;TRRTG&quot; || tran == &quot;TRRMT&quot; || tran == &quot;TRSKN&quot;) {
    var datax = {
      tran: &quot;transfer&quot;,
      apa: tran.substr(0, 2).toLowerCase() + tran.charAt(2) + tran.slice(3).toLowerCase(),
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/transfer/get-transfer&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        if (tran == &quot;TROVB&quot;) {
          cod = 6; //73
        } else if (tran == &quot;TRRTG&quot;) {
          cod = res.sum_dana == 2 ? 12 : 8;
        } else if (tran == &quot;TRSKN&quot;) {
          cod = res.sum_dana == 2 ? 2 : 3;
        } else if (tran == &quot;TRRMT&quot;) {
          cod = 16;
        }

        jsondata = {
          ...res,
          jenisnasabah1: res.jenisnasabah1 == 1 ? &quot;I&quot; : &quot;P&quot;,
          no_id_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.idno1,
          nama_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname1,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.idtype1,
          // Natasha [22-Feb-2024]
          // jenis_pelaku: res.hasOwnProperty(&quot;wic&quot;) &amp;&amp; res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          isWic: res.hasOwnProperty(&quot;wic&quot;) ? res.wic.wic_cust : &quot;N&quot;,
          isWkl: res.kua.kua_cust,
        };

        fast.tranNext(cod, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;STTUN&quot;) {
    var datax = {
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/setor-tunai/get-setor-tunai&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        jsondata = {
          ...res,
          prodname2: res.accountType,
          idtype2: res.typeid,
          no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.idno2,
          // no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : res.kua.kua_cust == &quot;Y&quot; ? res.kua_id_no : res.idno2,
          nama_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname2,
          // Natasha [22-Feb-2024]
          // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
          isWic: res.wic.wic_cust,
          isWkl: res.kua.kua_cust,
        };
        console.log(`sttun ===`, jsondata);
        fast.tranNext(1, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;TRTUN&quot;) {
    var datax = {
      no_ref: ref,
    };
    fast.getURL(&quot;/fastchannel/tarik-tunai/get-tarik-tunai&quot;, datax, function (err, res) {
      if (res.no_ref != undefined) {
        jsondata = {
          ...res,
          prodname1: res.accountType,
          idtype1: res.typeid,
          no_id_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_id_no : `${res[&quot;idno1&quot;]}`.replace(/^\s+|\s+$/gm, &quot;&quot;),
          nama_pelaku: res.wic.wic_cust == &quot;Y&quot; ? res.wic.wic_name : res.kua.kua_cust == &quot;Y&quot; ? res.kua_name : res.custname1,
          // Natasha [22-Feb-2024]
          //  jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
          idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
          isWic: res.wic.wic_cust,
          isWkl: res.kua.kua_cust,
        };
        fast.tranNext(4, jsondata);
      } else {
        window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
      }
    });
  } else if (tran == &quot;BYBNT&quot;) {
    fast.getURL(&quot;/fastchannel/bank-notes/get-bn/bybnt/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(58, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  } else if (tran == &quot;SLBNT&quot;) {
    fast.getURL(&quot;/fastchannel/bank-notes/get-bn/slbnt/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(57, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  } else if (tran == &quot;PAYCC&quot;) {
    fast.getURL(&quot;/fastchannel/pay-mbranch/findRef/&quot; + ref, {}, function (err, res) {
      try {
        if (res.no_ref != undefined) {
          jsondata = {
            ...res,
            noCC: res.noBill,
            idtype1: res.typeid,
            // Natasha [22-Feb-2024]
            // jenis_pelaku: res.wic.wic_cust == &quot;Y&quot; ? (res.hasOwnProperty(&quot;wici_id_type&quot;) ? &quot;WIC - Individu&quot; : &quot;WIC - Badan Usaha&quot;) : &quot;Nasabah&quot;,
            idtype_pelaku: res.hasOwnProperty(&quot;wici_id_type&quot;) ? res.wici_id_type : res.hasOwnProperty(&quot;wicn_id_type&quot;) ? res.wicn_id_type : res.typeid,
            isWic: res.wic.wic_cust,
            isWkl: res.kua.kua_cust,
          };
          fast.tranNext(8901, jsondata);
        } else {
          window.alert(&quot;Nomor referensi sudah tidak berlaku&quot;);
        }
      } catch (error) {
        console.log(error);
      }
    });
  }
}








        &quot;))]</value>
      <webElementGuid>5ff41867-4ec0-469d-91b9-97b457ec3d20</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
